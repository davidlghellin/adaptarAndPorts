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

    // 3. ADAPTADOR DE ENTRADA: API REST con Axum
    println!("🌐 Configurando adaptador de entrada (API REST)...");
    let app = api_rest::crear_router(empleado_service, reserva_service);

    // 4. Arrancar el servidor
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("\n✅ Servidor corriendo en http://{}", addr);
    println!("\n📖 Documentación API:");
    println!("   Swagger UI: http://{}/swagger-ui", addr);
    println!("   OpenAPI JSON: http://{}/api-docs/openapi.json", addr);
    println!("\n📚 Endpoints disponibles:");
    println!("   POST   /empleados               - Crear empleado");
    println!("   GET    /empleados               - Listar empleados");
    println!("   GET    /empleados/:id           - Obtener empleado");
    println!("   POST   /empleados/:id/activar   - Activar empleado");
    println!("   POST   /empleados/:id/desactivar - Desactivar empleado");
    println!();
    println!("   POST   /reservas                - Crear reserva");
    println!("   GET    /reservas                - Listar reservas");
    println!("   GET    /reservas/:id            - Obtener reserva");
    println!("   POST   /reservas/:id/confirmar  - Confirmar reserva");
    println!("   POST   /reservas/:id/cancelar   - Cancelar reserva");
    println!();
    println!("   GET    /empleados/:id/reservas  - Listar reservas de empleado");
    println!("   GET    /disponibilidad?fecha=YYYY-MM-DD - Tabla de disponibilidad");
    println!();
    println!("🎯 Presiona Ctrl+C para detener el servidor\n");

    axum::serve(listener, app).await.unwrap();
}
