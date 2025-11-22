// 🚀 BINARIO PRINCIPAL - Sistema de Reservas de Empleados
//
// Aquí es donde "armamos" toda la aplicación:
// 1. Elegimos qué adaptadores usar
// 2. Creamos las instancias
// 3. Conectamos todo mediante inyección de dependencias

use reservas_adapters::{InMemoryEmpleadoRepository, InMemoryReservaRepository};
use reservas_application::{EmpleadoServiceImpl, ReservaServiceImpl};
use reservas_domain::{DisponibilidadService, Slot};
use reservas_ports::{EmpleadoService, ReservaService};
use chrono::{Datelike, Timelike, Utc};

#[tokio::main]
async fn main() {
    println!("🎯 Sistema de Reservas de Empleados - Arquitectura Hexagonal\n");
    println!("📦 Nuevo modelo de dominio:");
    println!("   - Empleados con reservas de tiempo");
    println!("   - Slots de 1 hora (9:00-18:00)");
    println!("   - Tabla de disponibilidad");
    println!();

    // 1. Creamos los adaptadores
    let empleado_repo = InMemoryEmpleadoRepository::new();
    let reserva_repo = InMemoryReservaRepository::new();

    // 2. Creamos los servicios de aplicación
    let empleado_service = EmpleadoServiceImpl::new(empleado_repo);
    let reserva_service = ReservaServiceImpl::new(reserva_repo);

    // 3. Creamos empleados
    println!("👥 Creando empleados...");
    let emp1 = empleado_service
        .crear_empleado("Juan López".to_string(), "juan@empresa.com".to_string())
        .await
        .unwrap();

    let emp2 = empleado_service
        .crear_empleado("María García".to_string(), "maria@empresa.com".to_string())
        .await
        .unwrap();

    println!("   ✓ {} creado", emp1.nombre);
    println!("   ✓ {} creada\n", emp2.nombre);

    // 4. Creamos slots para mañana
    let mañana = Utc::now() + chrono::Duration::days(1);
    let slot_10 = Slot::from_date_and_hour(
        mañana.year(),
        mañana.month(),
        mañana.day(),
        10,
    ).unwrap();

    let slot_11 = Slot::from_date_and_hour(
        mañana.year(),
        mañana.month(),
        mañana.day(),
        11,
    ).unwrap();

    // 5. Creamos reservas
    println!("📝 Creando reservas...");

    match reserva_service
        .crear_reserva(
            emp1.id.clone(),
            slot_10.clone(),
            "Reunión con cliente importante".to_string(),
        )
        .await
    {
        Ok(r) => {
            println!("   ✓ Reserva creada para {} a las {}:00", emp1.nombre, r.slot.inicio.hour());
            println!("     Descripción: {}", r.descripcion);
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    match reserva_service
        .crear_reserva(
            emp2.id.clone(),
            slot_11.clone(),
            "Entrevista con candidato".to_string(),
        )
        .await
    {
        Ok(r) => {
            println!("   ✓ Reserva creada para {} a las {}:00", emp2.nombre, r.slot.inicio.hour());
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    println!();

    // 6. Intentar crear reserva duplicada (debe fallar)
    println!("🔒 Probando validación: reserva duplicada...");
    match reserva_service
        .crear_reserva(
            emp1.id.clone(),
            slot_10.clone(),
            "Otra reunión".to_string(),
        )
        .await
    {
        Ok(_) => println!("   ✗ ERROR: No debería haber permitido esto!"),
        Err(e) => println!("   ✓ Validación correcta: {}\n", e),
    }

    // 7. Generar tabla de disponibilidad
    println!("📊 Tabla de Disponibilidad:");
    let empleados = empleado_service.listar_empleados().await.unwrap();
    let reservas = reserva_service.listar_reservas().await.unwrap();

    let slots_del_dia: Vec<Slot> = (9..=12)
        .filter_map(|h| {
            Slot::from_date_and_hour(
                mañana.year(),
                mañana.month(),
                mañana.day(),
                h,
            )
        })
        .collect();

    let tabla = DisponibilidadService::generar_tabla_disponibilidad(
        &empleados,
        &slots_del_dia,
        &reservas,
    );

    println!("{}", tabla.formato_texto());

    // 8. Encontrar slots libres para reunión grupal
    let slots_libres = DisponibilidadService::slots_con_todos_disponibles(
        &empleados,
        &slots_del_dia,
        &reservas,
    );

    println!("\n🎯 Slots donde TODOS están disponibles:");
    for slot in &slots_libres {
        println!("   ✓ {}", slot.formato_legible());
    }

    println!("\n🎓 Ventajas de la arquitectura hexagonal:");
    println!("   1. ✅ Dominio rico con lógica compleja (DisponibilidadService)");
    println!("   2. ✅ Validaciones en el dominio (un empleado, un slot)");
    println!("   3. ✅ Fácil cambiar adaptadores (InMemory → Postgres)");
    println!("   4. ✅ Tests independientes por capa");
    println!("   5. ✅ El compilador protege la arquitectura");
}
