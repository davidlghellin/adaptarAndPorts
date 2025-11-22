// 🚀 BINARIO PRINCIPAL
//
// Aquí es donde "armamos" toda la aplicación:
// 1. Elegimos qué adaptadores usar
// 2. Creamos las instancias
// 3. Conectamos todo mediante inyección de dependencias

use reservas_adapters::InMemoryReservaRepository;
use reservas_application::ReservaServiceImpl;
use reservas_ports::ReservaService;
use chrono::Utc;

#[tokio::main]
async fn main() {
    println!("🎯 Sistema de Reservas - Arquitectura Hexagonal (Multi-crate)\n");
    println!("📦 Estructura:");
    println!("   - reservas-domain: Lógica de negocio pura");
    println!("   - reservas-ports: Interfaces (contratos)");
    println!("   - reservas-application: Casos de uso");
    println!("   - reservas-adapters: Implementaciones concretas");
    println!();

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

    println!("\n🎓 Ventajas de separar en crates:");
    println!("   1. ✅ El compilador FUERZA las dependencias correctas");
    println!("   2. ✅ Imposible que dominio dependa de infraestructura");
    println!("   3. ✅ Cada crate se puede versionar independientemente");
    println!("   4. ✅ Reutilización: otros proyectos pueden usar solo el dominio");
    println!("   5. ✅ Compilación en paralelo más eficiente");
}
