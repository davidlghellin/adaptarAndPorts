// Comandos del CLI

use crate::api_client::ApiClient;
use chrono::{NaiveDate, NaiveTime, TimeZone, Utc};
use colored::Colorize;
use tabled::{Table, Tabled};

// ============= Comandos de Empleados =============

#[derive(Tabled)]
struct EmpleadoRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Nombre")]
    nombre: String,
    #[tabled(rename = "Email")]
    email: String,
    #[tabled(rename = "Estado")]
    estado: String,
}

pub fn crear_empleado(client: &ApiClient, nombre: String, email: String) {
    println!("{}", "Creando empleado...".cyan());

    match client.crear_empleado(nombre, email) {
        Ok(empleado) => {
            println!("{}", "✓ Empleado creado exitosamente".green());
            println!("  ID: {}", empleado.id);
            println!("  Nombre: {}", empleado.nombre);
            println!("  Email: {}", empleado.email);
            println!("  Activo: {}", if empleado.activo { "Sí" } else { "No" });
        }
        Err(e) => {
            println!("{} {}", "✗ Error:".red(), e);
        }
    }
}

pub fn listar_empleados(client: &ApiClient) {
    println!("{}", "Obteniendo lista de empleados...".cyan());

    match client.listar_empleados() {
        Ok(empleados) => {
            if empleados.is_empty() {
                println!("{}", "No hay empleados registrados".yellow());
                return;
            }

            let rows: Vec<EmpleadoRow> = empleados
                .into_iter()
                .map(|e| EmpleadoRow {
                    id: e.id,
                    nombre: e.nombre,
                    email: e.email,
                    estado: if e.activo {
                        "Activo".to_string()
                    } else {
                        "Inactivo".to_string()
                    },
                })
                .collect();

            let count = rows.len();
            let table = Table::new(rows).to_string();
            println!("\n{}", table);
            println!("\n{} empleado(s) encontrado(s)", count);
        }
        Err(e) => {
            println!("{} {}", "✗ Error:".red(), e);
        }
    }
}

pub fn obtener_empleado(client: &ApiClient, id: String) {
    println!("{}", "Obteniendo empleado...".cyan());

    match client.obtener_empleado(&id) {
        Ok(empleado) => {
            println!("\n{}", "Información del empleado:".green());
            println!("  ID: {}", empleado.id);
            println!("  Nombre: {}", empleado.nombre);
            println!("  Email: {}", empleado.email);
            println!("  Activo: {}", if empleado.activo { "Sí" } else { "No" });
        }
        Err(e) => {
            println!("{} {}", "✗ Error:".red(), e);
        }
    }
}

pub fn activar_empleado(client: &ApiClient, id: String) {
    println!("{}", "Activando empleado...".cyan());

    match client.activar_empleado(&id) {
        Ok(empleado) => {
            println!("{}", "✓ Empleado activado exitosamente".green());
            println!("  {}: {}", "Nombre".bold(), empleado.nombre);
        }
        Err(e) => {
            println!("{} {}", "✗ Error:".red(), e);
        }
    }
}

pub fn desactivar_empleado(client: &ApiClient, id: String) {
    println!("{}", "Desactivando empleado...".cyan());

    match client.desactivar_empleado(&id) {
        Ok(empleado) => {
            println!("{}", "✓ Empleado desactivado exitosamente".green());
            println!("  {}: {}", "Nombre".bold(), empleado.nombre);
        }
        Err(e) => {
            println!("{} {}", "✗ Error:".red(), e);
        }
    }
}

// ============= Comandos de Reservas =============

#[derive(Tabled)]
struct ReservaRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Empleado ID")]
    empleado_id: String,
    #[tabled(rename = "Fecha")]
    fecha: String,
    #[tabled(rename = "Horario")]
    horario: String,
    #[tabled(rename = "Descripción")]
    descripcion: String,
    #[tabled(rename = "Estado")]
    estado: String,
}

pub fn crear_reserva(
    client: &ApiClient,
    empleado_id: String,
    fecha: String,
    hora: u32,
    descripcion: String,
) {
    println!("{}", "Creando reserva...".cyan());

    // Parsear fecha
    let date = match NaiveDate::parse_from_str(&fecha, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            println!("{} Formato de fecha inválido. Use YYYY-MM-DD", "✗".red());
            return;
        }
    };

    let time = match NaiveTime::from_hms_opt(hora, 0, 0) {
        Some(t) => t,
        None => {
            println!("{} Hora inválida", "✗".red());
            return;
        }
    };

    let datetime = Utc.from_utc_datetime(&date.and_time(time));

    match client.crear_reserva(empleado_id, datetime, descripcion) {
        Ok(reserva) => {
            println!("{}", "✓ Reserva creada exitosamente".green());
            println!("  ID: {}", reserva.id);
            println!("  Slot: {} - {}",
                reserva.slot_inicio.format("%Y-%m-%d %H:%M"),
                reserva.slot_fin.format("%H:%M")
            );
            println!("  Descripción: {}", reserva.descripcion);
            println!("  Estado: {}", reserva.estado);
        }
        Err(e) => {
            println!("{} {}", "✗ Error:".red(), e);
        }
    }
}

pub fn listar_reservas(client: &ApiClient) {
    println!("{}", "Obteniendo lista de reservas...".cyan());

    match client.listar_reservas() {
        Ok(reservas) => {
            if reservas.is_empty() {
                println!("{}", "No hay reservas registradas".yellow());
                return;
            }

            let rows: Vec<ReservaRow> = reservas
                .into_iter()
                .map(|r| ReservaRow {
                    id: r.id,
                    empleado_id: r.empleado_id,
                    fecha: r.slot_inicio.format("%Y-%m-%d").to_string(),
                    horario: format!("{}-{}",
                        r.slot_inicio.format("%H:%M"),
                        r.slot_fin.format("%H:%M")
                    ),
                    descripcion: r.descripcion,
                    estado: r.estado,
                })
                .collect();

            let count = rows.len();
            let table = Table::new(rows).to_string();
            println!("\n{}", table);
            println!("\n{} reserva(s) encontrada(s)", count);
        }
        Err(e) => {
            println!("{} {}", "✗ Error:".red(), e);
        }
    }
}

pub fn listar_reservas_empleado(client: &ApiClient, empleado_id: String) {
    println!("{}", format!("Obteniendo reservas del empleado {}...", empleado_id).cyan());

    match client.listar_reservas_empleado(&empleado_id) {
        Ok(reservas) => {
            if reservas.is_empty() {
                println!("{}", "Este empleado no tiene reservas".yellow());
                return;
            }

            let rows: Vec<ReservaRow> = reservas
                .into_iter()
                .map(|r| ReservaRow {
                    id: r.id,
                    empleado_id: r.empleado_id.clone(),
                    fecha: r.slot_inicio.format("%Y-%m-%d").to_string(),
                    horario: format!("{}-{}",
                        r.slot_inicio.format("%H:%M"),
                        r.slot_fin.format("%H:%M")
                    ),
                    descripcion: r.descripcion,
                    estado: r.estado,
                })
                .collect();

            let count = rows.len();
            let table = Table::new(rows).to_string();
            println!("\n{}", table);
            println!("\n{} reserva(s) encontrada(s)", count);
        }
        Err(e) => {
            println!("{} {}", "✗ Error:".red(), e);
        }
    }
}

pub fn confirmar_reserva(client: &ApiClient, id: String) {
    println!("{}", "Confirmando reserva...".cyan());

    match client.confirmar_reserva(&id) {
        Ok(reserva) => {
            println!("{}", "✓ Reserva confirmada exitosamente".green());
            println!("  Estado: {}", reserva.estado);
        }
        Err(e) => {
            println!("{} {}", "✗ Error:".red(), e);
        }
    }
}

pub fn cancelar_reserva(client: &ApiClient, id: String) {
    println!("{}", "Cancelando reserva...".cyan());

    match client.cancelar_reserva(&id) {
        Ok(reserva) => {
            println!("{}", "✓ Reserva cancelada exitosamente".green());
            println!("  Estado: {}", reserva.estado);
        }
        Err(e) => {
            println!("{} {}", "✗ Error:".red(), e);
        }
    }
}

// ============= Comandos de Disponibilidad =============

pub fn ver_disponibilidad(client: &ApiClient, fecha: String) {
    println!("{}", format!("Obteniendo disponibilidad para {}...", fecha).cyan());

    match client.obtener_disponibilidad(&fecha) {
        Ok(tabla) => {
            println!("\n{}", format!("Disponibilidad - {}", tabla.fecha).green().bold());
            println!("\n{}", "Slots disponibles:".bold());

            for slot in &tabla.slots {
                println!("  {}:00-{}:00", slot.hora, slot.hora + 1);
            }

            println!("\n{}", "Por empleado:".bold());

            // Agrupar por empleado
            let mut por_empleado: std::collections::HashMap<String, Vec<_>> = std::collections::HashMap::new();

            for disp in &tabla.disponibilidad {
                por_empleado
                    .entry(disp.empleado_nombre.clone())
                    .or_default()
                    .push(disp);
            }

            for (nombre, slots) in por_empleado {
                println!("\n  {}:", nombre.bold());
                for slot in slots {
                    let simbolo = if slot.disponible {
                        "✓".green()
                    } else {
                        "✗".red()
                    };

                    let info = if slot.disponible {
                        "Disponible".to_string()
                    } else {
                        format!("Ocupado: {}",
                            slot.descripcion_reserva.as_ref().unwrap_or(&"Sin descripción".to_string()))
                    };

                    println!("    {} {}:00 - {}",
                        simbolo,
                        slot.slot_inicio.format("%H"),
                        info
                    );
                }
            }
        }
        Err(e) => {
            println!("{} {}", "✗ Error:".red(), e);
        }
    }
}
