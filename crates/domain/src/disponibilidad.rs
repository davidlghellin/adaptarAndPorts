use crate::{Empleado, Reserva, Slot};
use chrono::{Datelike, Timelike};
use std::collections::HashMap;

/// Servicio de Dominio: Gestiona la disponibilidad de empleados
///
/// Este es un SERVICIO DE DOMINIO porque:
/// - Coordina múltiples entidades (Empleado, Reserva, Slot)
/// - No pertenece naturalmente a ninguna entidad
/// - Encapsula lógica de negocio compleja
pub struct DisponibilidadService;

/// Representa la disponibilidad de un empleado en un slot
#[derive(Debug, Clone, PartialEq)]
pub struct DisponibilidadSlot {
    pub empleado_id: String,
    pub empleado_nombre: String,
    pub slot: Slot,
    pub disponible: bool,
    pub reserva_id: Option<String>,
    pub descripcion_reserva: Option<String>,
}

/// Tabla de disponibilidad para múltiples empleados
#[derive(Debug)]
pub struct TablaDisponibilidad {
    pub slots: Vec<Slot>,
    pub empleados: Vec<Empleado>,
    pub disponibilidad: Vec<DisponibilidadSlot>,
}

impl DisponibilidadService {
    /// Verifica si un empleado tiene disponibilidad en un slot específico
    ///
    /// Regla de negocio: Un empleado solo puede tener UNA reserva activa por slot
    pub fn empleado_disponible_en_slot(
        empleado_id: &str,
        slot: &Slot,
        reservas: &[Reserva],
    ) -> bool {
        !reservas
            .iter()
            .any(|r| r.empleado_id == empleado_id && r.slot == *slot && r.esta_activa())
    }

    /// Obtiene todas las reservas de un empleado en un rango de slots
    pub fn reservas_de_empleado<'a>(
        empleado_id: &str,
        reservas: &'a [Reserva],
    ) -> Vec<&'a Reserva> {
        reservas
            .iter()
            .filter(|r| r.empleado_id == empleado_id && r.esta_activa())
            .collect()
    }

    /// Genera una tabla de disponibilidad para un conjunto de empleados y slots
    pub fn generar_tabla_disponibilidad(
        empleados: &[Empleado],
        slots: &[Slot],
        reservas: &[Reserva],
    ) -> TablaDisponibilidad {
        let mut disponibilidad = Vec::new();

        for empleado in empleados.iter().filter(|e| e.activo) {
            for slot in slots {
                let reserva = reservas
                    .iter()
                    .find(|r| r.empleado_id == empleado.id && r.slot == *slot && r.esta_activa());

                disponibilidad.push(DisponibilidadSlot {
                    empleado_id: empleado.id.clone(),
                    empleado_nombre: empleado.nombre.clone(),
                    slot: slot.clone(),
                    disponible: reserva.is_none(),
                    reserva_id: reserva.map(|r| r.id.clone()),
                    descripcion_reserva: reserva.map(|r| r.descripcion.clone()),
                });
            }
        }

        TablaDisponibilidad {
            slots: slots.to_vec(),
            empleados: empleados.iter().filter(|e| e.activo).cloned().collect(),
            disponibilidad,
        }
    }

    /// Encuentra los slots libres para un empleado específico
    pub fn slots_libres_empleado(
        empleado_id: &str,
        slots: &[Slot],
        reservas: &[Reserva],
    ) -> Vec<Slot> {
        slots
            .iter()
            .filter(|slot| Self::empleado_disponible_en_slot(empleado_id, slot, reservas))
            .cloned()
            .collect()
    }

    /// Encuentra slots donde TODOS los empleados están disponibles
    pub fn slots_con_todos_disponibles(
        empleados: &[Empleado],
        slots: &[Slot],
        reservas: &[Reserva],
    ) -> Vec<Slot> {
        let empleados_activos: Vec<_> = empleados.iter().filter(|e| e.activo).collect();

        slots
            .iter()
            .filter(|slot| {
                empleados_activos
                    .iter()
                    .all(|emp| Self::empleado_disponible_en_slot(&emp.id, slot, reservas))
            })
            .cloned()
            .collect()
    }

    /// Resumen de ocupación: cuántos empleados tienen reserva en cada slot
    pub fn resumen_ocupacion(slots: &[Slot], reservas: &[Reserva]) -> HashMap<Slot, usize> {
        let mut ocupacion = HashMap::new();

        for slot in slots {
            let count = reservas
                .iter()
                .filter(|r| r.slot == *slot && r.esta_activa())
                .count();
            ocupacion.insert(slot.clone(), count);
        }

        ocupacion
    }

    /// Obtiene los empleados más ocupados (con más reservas)
    pub fn empleados_mas_ocupados(
        empleados: &[Empleado],
        reservas: &[Reserva],
    ) -> Vec<(String, usize)> {
        let mut conteo: HashMap<String, usize> = HashMap::new();

        for reserva in reservas.iter().filter(|r| r.esta_activa()) {
            *conteo.entry(reserva.empleado_id.clone()).or_insert(0) += 1;
        }

        let mut resultado: Vec<_> = empleados
            .iter()
            .filter(|e| e.activo)
            .map(|e| (e.nombre.clone(), *conteo.get(&e.id).unwrap_or(&0)))
            .collect();

        resultado.sort_by(|a, b| b.1.cmp(&a.1));
        resultado
    }
}

impl TablaDisponibilidad {
    /// Renderiza la tabla en formato de texto
    pub fn formato_texto(&self) -> String {
        let mut resultado = String::new();

        // Encabezado con slots
        resultado.push_str(&format!("{:<20}", "Empleado"));
        for slot in &self.slots {
            resultado.push_str(&format!(" | {:^12}", slot.inicio.hour()));
        }
        resultado.push('\n');

        // Línea separadora
        resultado.push_str(&"-".repeat(20 + (self.slots.len() * 15)));
        resultado.push('\n');

        // Filas por empleado
        for empleado in &self.empleados {
            resultado.push_str(&format!("{:<20}", empleado.nombre));

            for slot in &self.slots {
                let disp = self
                    .disponibilidad
                    .iter()
                    .find(|d| d.empleado_id == empleado.id && d.slot == *slot);

                let simbolo = match disp {
                    Some(d) if d.disponible => " ✓ ",
                    Some(_) => " ✗ ",
                    None => " ? ",
                };

                resultado.push_str(&format!(" | {:^12}", simbolo));
            }
            resultado.push('\n');
        }

        resultado
    }

    /// Obtiene disponibilidad de un empleado en un slot específico
    pub fn get_disponibilidad(
        &self,
        empleado_id: &str,
        slot: &Slot,
    ) -> Option<&DisponibilidadSlot> {
        self.disponibilidad
            .iter()
            .find(|d| d.empleado_id == empleado_id && d.slot == *slot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn crear_empleado(id: &str, nombre: &str) -> Empleado {
        Empleado::new(
            id.to_string(),
            nombre.to_string(),
            format!("{}@empresa.com", id),
        )
    }

    fn crear_slot_futuro(hour: u32) -> Slot {
        let manyana = Utc::now() + chrono::Duration::days(1);
        Slot::from_date_and_hour(manyana.year(), manyana.month(), manyana.day(), hour).unwrap()
    }

    #[test]
    fn test_empleado_disponible_sin_reservas() {
        let slot = crear_slot_futuro(10);
        let reservas = vec![];

        assert!(DisponibilidadService::empleado_disponible_en_slot(
            "emp-001", &slot, &reservas
        ));
    }

    #[test]
    fn test_empleado_no_disponible_con_reserva() {
        let slot = crear_slot_futuro(10);
        let reserva = Reserva::new(
            "r1".to_string(),
            "emp-001".to_string(),
            slot.clone(),
            "Reunión".to_string(),
        )
        .unwrap();

        assert!(!DisponibilidadService::empleado_disponible_en_slot(
            "emp-001",
            &slot,
            &[reserva]
        ));
    }

    #[test]
    fn test_generar_tabla_disponibilidad() {
        let empleados = vec![
            crear_empleado("emp-001", "Juan"),
            crear_empleado("emp-002", "María"),
        ];

        let slots = vec![crear_slot_futuro(10), crear_slot_futuro(11)];

        let reserva = Reserva::new(
            "r1".to_string(),
            "emp-001".to_string(),
            slots[0].clone(),
            "Reunión".to_string(),
        )
        .unwrap();

        let tabla =
            DisponibilidadService::generar_tabla_disponibilidad(&empleados, &slots, &[reserva]);

        assert_eq!(tabla.empleados.len(), 2);
        assert_eq!(tabla.slots.len(), 2);
        assert_eq!(tabla.disponibilidad.len(), 4); // 2 empleados x 2 slots

        // emp-001 a las 10:00 debe estar ocupado
        let disp = tabla.get_disponibilidad("emp-001", &slots[0]).unwrap();
        assert!(!disp.disponible);

        // emp-002 a las 10:00 debe estar libre
        let disp = tabla.get_disponibilidad("emp-002", &slots[0]).unwrap();
        assert!(disp.disponible);
    }

    #[test]
    fn test_slots_con_todos_disponibles() {
        let empleados = vec![
            crear_empleado("emp-001", "Juan"),
            crear_empleado("emp-002", "María"),
        ];

        let slots = vec![crear_slot_futuro(10), crear_slot_futuro(11)];

        // Solo emp-001 tiene reserva a las 10:00
        let reserva = Reserva::new(
            "r1".to_string(),
            "emp-001".to_string(),
            slots[0].clone(),
            "Reunión".to_string(),
        )
        .unwrap();

        let libres =
            DisponibilidadService::slots_con_todos_disponibles(&empleados, &slots, &[reserva]);

        // Solo el slot de las 11:00 debe estar libre para ambos
        assert_eq!(libres.len(), 1);
        assert_eq!(libres[0], slots[1]);
    }
}
