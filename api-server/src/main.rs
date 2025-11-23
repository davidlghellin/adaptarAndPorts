// 🚀 SERVIDOR API REST - Punto de entrada del sistema
//
// Este es el binario que arranca el servidor HTTP.
// Aquí es donde "armamos" toda la aplicación con arquitectura hexagonal:
//
// 1. Creamos los ADAPTADORES DE SALIDA (repositorios in-memory)
// 2. Creamos los SERVICIOS DE APLICACIÓN (casos de uso)
// 3. Creamos el ADAPTADOR DE ENTRADA (API REST con Axum)
// 4. Conectamos todo mediante inyección de dependencias
// 5. Arrancamos el servidor

use reservas_adapters::{InMemoryEmpleadoRepository, InMemoryReservaRepository};
use reservas_application::{EmpleadoServiceImpl, ReservaServiceImpl};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("🚀 Sistema de Reservas - API REST");
    println!("📦 Arquitectura Hexagonal (Puertos y Adaptadores)\n");

    // 1. ADAPTADORES DE SALIDA: Repositorios en memoria
    println!("🔧 Configurando adaptadores de salida (repositorios)...");
    let empleado_repo = InMemoryEmpleadoRepository::new();
    let reserva_repo = InMemoryReservaRepository::new();

    // 2. SERVICIOS DE APLICACIÓN: Casos de uso
    println!("⚙️  Configurando servicios de aplicación...");
    let empleado_service = Arc::new(EmpleadoServiceImpl::new(empleado_repo))
        as Arc<dyn reservas_ports::EmpleadoService>;
    let reserva_service = Arc::new(ReservaServiceImpl::new(reserva_repo))
        as Arc<dyn reservas_ports::ReservaService>;

    // 3. ADAPTADORES DE ENTRADA: API REST + Web UI
    println!("🌐 Configurando adaptadores de entrada...");
    let api_router = api_rest::crear_router(
        Arc::clone(&empleado_service),
        Arc::clone(&reserva_service),
    );
    let web_router = web_ui::crear_router_web(
        Arc::clone(&empleado_service),
        Arc::clone(&reserva_service),
    );

    // Combinar ambos routers: Web UI en la raíz, API REST bajo /api
    let app = web_router.merge(axum::Router::new().nest("/api", api_router));

    // 4. Arrancar el servidor
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("\n✅ Servidor corriendo en http://{}", addr);
    println!("\n🌐 Interfaz Web:");
    println!("   http://{}/ - Interfaz web HTML", addr);
    println!("\n📖 Documentación API:");
    println!("   Swagger UI: http://{}/api/swagger-ui", addr);
    println!("   OpenAPI JSON: http://{}/api/api-docs/openapi.json", addr);
    println!("\n📚 API REST (bajo /api):");
    println!("   POST   /api/empleados               - Crear empleado");
    println!("   GET    /api/empleados               - Listar empleados");
    println!("   GET    /api/empleados/:id           - Obtener empleado");
    println!("   POST   /api/empleados/:id/activar   - Activar empleado");
    println!("   POST   /api/empleados/:id/desactivar - Desactivar empleado");
    println!();
    println!("   POST   /api/reservas                - Crear reserva");
    println!("   GET    /api/reservas                - Listar reservas");
    println!("   GET    /api/reservas/:id            - Obtener reserva");
    println!("   POST   /api/reservas/:id/confirmar  - Confirmar reserva");
    println!("   POST   /api/reservas/:id/cancelar   - Cancelar reserva");
    println!();
    println!("   GET    /api/empleados/:id/reservas  - Listar reservas de empleado");
    println!("   GET    /api/disponibilidad?fecha=YYYY-MM-DD - Tabla de disponibilidad");
    println!();
    println!("🎯 Presiona Ctrl+C para detener el servidor\n");

    axum::serve(listener, app).await.unwrap();
}
