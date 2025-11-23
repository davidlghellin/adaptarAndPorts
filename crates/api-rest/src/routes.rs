// Configuración de rutas de la API REST
// Aquí definimos todos los endpoints HTTP

use crate::{handlers, openapi::ApiDoc};
use axum::{
    routing::{get, post},
    Extension, Router,
};
use reservas_ports::{EmpleadoService, ReservaService, SalaService};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// Crea el router principal con todas las rutas de la API
pub fn crear_router(
    empleado_service: Arc<dyn EmpleadoService>,
    reserva_service: Arc<dyn ReservaService>,
    sala_service: Arc<dyn SalaService>,
) -> Router {
    let openapi = ApiDoc::openapi();

    Router::new()
        // Swagger UI - el path debe ser absoluto incluyendo /api
        .merge(SwaggerUi::new("/swagger-ui").url("/api/api-docs/openapi.json", openapi.clone()))
        // Ruta para servir el OpenAPI JSON
        .route(
            "/api-docs/openapi.json",
            get(|| async move { axum::Json(openapi) }),
        )
        // Rutas de empleados
        .route("/empleados", post(handlers::crear_empleado))
        .route("/empleados", get(handlers::listar_empleados))
        .route("/empleados/:id", get(handlers::obtener_empleado))
        .route(
            "/empleados/:id/desactivar",
            post(handlers::desactivar_empleado),
        )
        .route("/empleados/:id/activar", post(handlers::activar_empleado))
        // Rutas de reservas
        .route("/reservas", post(handlers::crear_reserva))
        .route("/reservas", get(handlers::listar_reservas))
        .route("/reservas/:id", get(handlers::obtener_reserva))
        .route("/reservas/:id/confirmar", post(handlers::confirmar_reserva))
        .route("/reservas/:id/cancelar", post(handlers::cancelar_reserva))
        // Reservas por empleado
        .route(
            "/empleados/:id/reservas",
            get(handlers::listar_reservas_empleado),
        )
        // Disponibilidad
        .route("/disponibilidad", get(handlers::obtener_disponibilidad))
        .route("/salas", get(handlers::listar_salas))
        .route("/salas", post(handlers::crear_sala))
        // Inyectar servicios como extensions (Dependency Injection)
        .layer(Extension(empleado_service))
        .layer(Extension(reserva_service))
        .layer(Extension(sala_service))
        // Logging de peticiones HTTP
        .layer(TraceLayer::new_for_http())
}
