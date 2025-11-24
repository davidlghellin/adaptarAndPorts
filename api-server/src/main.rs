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

use reservas_adapters::{
    InMemoryEmpleadoRepository, InMemoryReservaRepository, InMemorySalaRepository,
};
use reservas_application::{EmpleadoServiceImpl, ReservaServiceImpl, SalaServiceImpl};
use reservas_ports::r#in::reserva_service::ReservaService;
use std::sync::Arc;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use reservas_ports::r#in::empleado_service::EmpleadoService;
use reservas_ports::r#in::sala_service::SalaService;

#[tokio::main]
async fn main() {
    // Inicializar el sistema de logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("🚀 Sistema de Reservas - API REST");
    info!("📦 Arquitectura Hexagonal (Puertos y Adaptadores)");

    // 1. ADAPTADORES DE SALIDA: Repositorios en memoria
    info!("🔧 Configurando adaptadores de salida (repositorios)");
    let empleado_repo: InMemoryEmpleadoRepository = InMemoryEmpleadoRepository::new();
    let reserva_repo: InMemoryReservaRepository = InMemoryReservaRepository::new();
    let sala_repository: InMemorySalaRepository = InMemorySalaRepository::new();

    // 2. SERVICIOS DE APLICACIÓN: Casos de uso
    info!("⚙️  Configurando servicios de aplicación");
    let empleado_service: Arc<dyn EmpleadoService> =
        Arc::new(EmpleadoServiceImpl::new(empleado_repo)) as Arc<dyn EmpleadoService>;
    let reserva_service: Arc<dyn ReservaService> =
        Arc::new(ReservaServiceImpl::new(reserva_repo)) as Arc<dyn ReservaService>;
    let sala_service: Arc<dyn SalaService> =
        Arc::new(SalaServiceImpl::new(sala_repository)) as Arc<dyn SalaService>;

    // 3. ADAPTADORES DE ENTRADA: API REST + Web UI
    info!("🌐 Configurando adaptadores de entrada");
    let api_router = api_rest::crear_router(
        Arc::clone(&empleado_service),
        Arc::clone(&reserva_service),
        Arc::clone(&sala_service),
    );
    let web_router = web_ui::crear_router_web(
        Arc::clone(&empleado_service),
        Arc::clone(&reserva_service),
        Arc::clone(&sala_service),
    );
    // Combinar ambos routers: Web UI en la raíz, API REST bajo /api
    let app = web_router.merge(axum::Router::new().nest("/api", api_router));

    // 4. Arrancar el servidor
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("✅ Servidor corriendo en http://{}", addr);
    info!("🌐 Interfaz Web: http://{}/", addr);
    info!("📖 Swagger UI: http://{}/api/swagger-ui", addr);
    info!("📖 OpenAPI JSON: http://{}/api/api-docs/openapi.json", addr);
    warn!("🎯 Presiona Ctrl+C para detener el servidor");

    axum::serve(listener, app).await.unwrap();
}
