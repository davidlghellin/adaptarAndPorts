// Ejemplo de uso del servicio de disponibilidad
// Ejecutar con: cargo run --example tabla_disponibilidad -p reservas-domain

use chrono::{Datelike, Timelike, Utc};
use reservas_domain::{DisponibilidadService, Empleado, Reserva, Slot};

fn main() {
    println!("📊 Demo: Tabla de Disponibilidad de Empleados\n");

    // 1. Crear empleados
    let empleados = vec![
        Empleado::new(
            "emp-001".to_string(),
            "Juan López".to_string(),
            "juan@empresa.com".to_string(),
        ),
        Empleado::new(
            "emp-002".to_string(),
            "María García".to_string(),
            "maria@empresa.com".to_string(),
        ),
        Empleado::new(
            "emp-003".to_string(),
            "Pedro Martínez".to_string(),
            "pedro@empresa.com".to_string(),
        ),
    ];

    println!("👥 Empleados:");
    for emp in &empleados {
        println!("   - {} ({})", emp.nombre, emp.id);
    }
    println!();

    // 2. Crear slots del día (de 9:00 a 12:00)
    let manyana = Utc::now() + chrono::Duration::days(1);
    let slots: Vec<Slot> = (9..=12)
        .filter_map(|h| Slot::from_date_and_hour(manyana.year(), manyana.month(), manyana.day(), h))
        .collect();

    println!("⏰ Slots disponibles:");
    for slot in &slots {
        println!("   - {}", slot.formato_legible());
    }
    println!();

    // 3. Crear algunas reservas
    let mut reservas = Vec::new();

    // Juan tiene reunión a las 9:00
    if let Ok(r) = Reserva::new(
        "r1".to_string(),
        "emp-001".to_string(),
        slots[0].clone(),
        "Reunión de equipo".to_string(),
    ) {
        reservas.push(r);
    }

    // María tiene reunión a las 9:00 también
    if let Ok(r) = Reserva::new(
        "r2".to_string(),
        "emp-002".to_string(),
        slots[0].clone(),
        "Entrevista con candidato".to_string(),
    ) {
        reservas.push(r);
    }

    // Pedro tiene reunión a las 11:00
    if let Ok(r) = Reserva::new(
        "r3".to_string(),
        "emp-003".to_string(),
        slots[2].clone(),
        "Presentación al cliente".to_string(),
    ) {
        reservas.push(r);
    }

    println!("📅 Reservas creadas:");
    for r in &reservas {
        let emp = empleados.iter().find(|e| e.id == r.empleado_id).unwrap();
        println!(
            "   - {} a las {}:00 - {}",
            emp.nombre,
            r.slot.inicio.hour(),
            r.descripcion
        );
    }
    println!();

    // 4. Generar tabla de disponibilidad
    let tabla = DisponibilidadService::generar_tabla_disponibilidad(&empleados, &slots, &reservas);

    println!("📊 Tabla de Disponibilidad:");
    println!("{}", tabla.formato_texto());
    println!("   ✓ = Disponible");
    println!("   ✗ = Ocupado");
    println!();

    // 5. Encontrar slots donde todos están disponibles
    let libres_todos =
        DisponibilidadService::slots_con_todos_disponibles(&empleados, &slots, &reservas);

    println!("🎯 Slots donde TODOS están disponibles:");
    if libres_todos.is_empty() {
        println!("   ⚠️  No hay slots donde todos estén disponibles");
    } else {
        for slot in &libres_todos {
            println!("   ✓ {}", slot.formato_legible());
        }
    }
    println!();

    // 6. Ver slots libres de un empleado específico
    let emp_id = "emp-001";
    let libres_juan = DisponibilidadService::slots_libres_empleado(emp_id, &slots, &reservas);

    println!("📌 Slots libres para Juan López:");
    for slot in &libres_juan {
        println!("   ✓ {}", slot.formato_legible());
    }
    println!();

    // 7. Resumen de ocupación
    let ocupacion = DisponibilidadService::resumen_ocupacion(&slots, &reservas);

    println!("📈 Resumen de ocupación:");
    for slot in &slots {
        let count = ocupacion.get(slot).unwrap_or(&0);
        println!(
            "   {} - {} empleado(s) ocupado(s)",
            slot.formato_legible(),
            count
        );
    }
    println!();

    // 8. Empleados más ocupados
    let ranking = DisponibilidadService::empleados_mas_ocupados(&empleados, &reservas);

    println!("🏆 Ranking de ocupación:");
    for (idx, (nombre, count)) in ranking.iter().enumerate() {
        println!("   {}. {} - {} reserva(s)", idx + 1, nombre, count);
    }
}
