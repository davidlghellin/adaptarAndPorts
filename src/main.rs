// Punto de entrada de la aplicación
// Aquí "armamos" todo conectando los adaptadores con los puertos

mod adapters;
mod application;
mod domain;
mod ports;

use adapters::repository_in_memory::InMemoryReservaRepository;
use application::ReservaServiceImpl;
use chrono::Utc;
use ports::ReservaService;

#[tokio::main]
async fn main() {
    println!("🎯 Sistema de Reservas - Arquitectura Hexagonal\n");

    // 1. Creamos el adaptador (repositorio en memoria)
    let repository = InMemoryReservaRepository::new();

    // 2. Inyectamos el adaptador en el servicio de aplicación
    let service = ReservaServiceImpl::new(repository);

    // 3. Usamos el servicio a través del puerto (interfaz)
    println!("📝 Creando una reserva...");
    let fecha_reserva = Utc::now() + chrono::Duration::days(3);

    match service
        .crear_reserva("Juan Pérez".to_string(), fecha_reserva, 4)
        .await
    {
        Ok(reserva) => {
            println!("✅ Reserva creada exitosamente!");
            println!("   ID: {}", reserva.id);
            println!("   Cliente: {}", reserva.nombre_cliente);
            println!("   Personas: {}", reserva.num_personas);
            println!("   Estado: {:?}\n", reserva.estado);

            // Confirmamos la reserva
            println!("✓ Confirmando reserva...");
            match service.confirmar_reserva(&reserva.id).await {
                Ok(reserva_confirmada) => {
                    println!("✅ Reserva confirmada!");
                    println!("   Estado: {:?}\n", reserva_confirmada.estado);
                }
                Err(e) => println!("❌ Error: {}", e),
            }

            // Listamos todas las reservas
            println!("📋 Lista de todas las reservas:");
            match service.listar_reservas().await {
                Ok(reservas) => {
                    for r in reservas {
                        println!("   - {} | {} personas | {:?}", r.nombre_cliente, r.num_personas, r.estado);
                    }
                }
                Err(e) => println!("❌ Error: {}", e),
            }
        }
        Err(e) => println!("❌ Error al crear reserva: {}", e),
    }

    println!("\n🎓 Ventajas de la arquitectura hexagonal:");
    println!("   1. El dominio (Reserva) no conoce nada de infraestructura");
    println!("   2. Podemos cambiar el adaptador (InMemory → Postgres) sin tocar el dominio");
    println!("   3. Los puertos definen contratos claros");
    println!("   4. Fácil de testear cada capa independientemente");
}
