use crate::slot::Slot;
use chrono::Utc;

/// Entidad de dominio: Reserva
/// Ahora representa una reserva de un EMPLEADO para un SLOT de tiempo específico
#[derive(Debug, Clone, PartialEq)]
pub struct Reserva {
    pub id: String,
    pub empleado_id: String,
    pub slot: Slot,
    pub descripcion: String,
    pub estado: EstadoReserva,
}

/// Estados posibles de una reserva
#[derive(Debug, Clone, PartialEq)]
pub enum EstadoReserva {
    Pendiente,
    Confirmada,
    Cancelada,
}

/// Errores del dominio
#[derive(Debug, PartialEq)]
pub enum ReservaError {
    SlotEnElPasado,
    SlotFueraDeHorarioLaboral,
    EmpleadoYaTieneReservaEnEsteSlot,
    DescripcionVacia,
}

impl Reserva {
    /// Constructor con validaciones de negocio
    pub fn new(
        id: String,
        empleado_id: String,
        slot: Slot,
        descripcion: String,
    ) -> Result<Self, ReservaError> {
        // Regla de negocio: no se pueden hacer reservas en el pasado
        if slot.inicio < Utc::now() {
            return Err(ReservaError::SlotEnElPasado);
        }

        // Regla de negocio: solo en horario laboral
        if !slot.es_horario_laboral() {
            return Err(ReservaError::SlotFueraDeHorarioLaboral);
        }

        // Regla de negocio: la descripción no puede estar vacía
        if descripcion.trim().is_empty() {
            return Err(ReservaError::DescripcionVacia);
        }

        Ok(Reserva {
            id,
            empleado_id,
            slot,
            descripcion,
            estado: EstadoReserva::Pendiente,
        })
    }

    /// Lógica de negocio: confirmar reserva
    pub fn confirmar(&mut self) {
        self.estado = EstadoReserva::Confirmada;
    }

    /// Lógica de negocio: cancelar reserva
    pub fn cancelar(&mut self) {
        self.estado = EstadoReserva::Cancelada;
    }

    /// Verifica si la reserva está activa (no cancelada)
    pub fn esta_activa(&self) -> bool {
        self.estado != EstadoReserva::Cancelada
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_crear_reserva_valida() {
        let manyana = Utc::now() + chrono::Duration::days(1);
        let slot =
            Slot::from_date_and_hour(manyana.year(), manyana.month(), manyana.day(), 10).unwrap();

        let reserva = Reserva::new(
            "1".to_string(),
            "emp-001".to_string(),
            slot,
            "Reunión con cliente".to_string(),
        );

        assert!(reserva.is_ok());
        let r = reserva.unwrap();
        assert_eq!(r.estado, EstadoReserva::Pendiente);
        assert_eq!(r.empleado_id, "emp-001");
    }

    #[test]
    fn test_slot_en_el_pasado() {
        let ayer = Utc::now() - chrono::Duration::days(1);
        let slot = Slot::from_date_and_hour(ayer.year(), ayer.month(), ayer.day(), 10).unwrap();

        let reserva = Reserva::new(
            "1".to_string(),
            "emp-001".to_string(),
            slot,
            "Reunión".to_string(),
        );

        assert_eq!(reserva, Err(ReservaError::SlotEnElPasado));
    }

    #[test]
    fn test_slot_fuera_horario_laboral() {
        let manyana = Utc::now() + chrono::Duration::days(1);
        let slot = Slot::from_date_and_hour(
            manyana.year(),
            manyana.month(),
            manyana.day(),
            20, // 8 PM - fuera de horario
        )
        .unwrap();

        let reserva = Reserva::new(
            "1".to_string(),
            "emp-001".to_string(),
            slot,
            "Reunión".to_string(),
        );

        assert_eq!(reserva, Err(ReservaError::SlotFueraDeHorarioLaboral));
    }

    #[test]
    fn test_descripcion_vacia() {
        let manyana = Utc::now() + chrono::Duration::days(1);
        let slot =
            Slot::from_date_and_hour(manyana.year(), manyana.month(), manyana.day(), 10).unwrap();

        let reserva = Reserva::new(
            "1".to_string(),
            "emp-001".to_string(),
            slot,
            "   ".to_string(), // solo espacios
        );

        assert_eq!(reserva, Err(ReservaError::DescripcionVacia));
    }

    #[test]
    fn test_confirmar_reserva() {
        let manyana = Utc::now() + chrono::Duration::days(1);
        let slot =
            Slot::from_date_and_hour(manyana.year(), manyana.month(), manyana.day(), 14).unwrap();

        let mut reserva = Reserva::new(
            "1".to_string(),
            "emp-001".to_string(),
            slot,
            "Reunión importante".to_string(),
        )
        .unwrap();

        reserva.confirmar();
        assert_eq!(reserva.estado, EstadoReserva::Confirmada);
        assert!(reserva.esta_activa());
    }

    #[test]
    fn test_cancelar_reserva() {
        let manyana = Utc::now() + chrono::Duration::days(1);
        let slot =
            Slot::from_date_and_hour(manyana.year(), manyana.month(), manyana.day(), 14).unwrap();

        let mut reserva = Reserva::new(
            "1".to_string(),
            "emp-001".to_string(),
            slot,
            "Reunión".to_string(),
        )
        .unwrap();

        reserva.cancelar();
        assert_eq!(reserva.estado, EstadoReserva::Cancelada);
        assert!(!reserva.esta_activa());
    }
}
