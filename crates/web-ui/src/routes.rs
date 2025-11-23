// Configuración de rutas para la interfaz web

use crate::handlers;
use axum::{
    routing::{get, post},
    Router,
};
use reservas_ports::{EmpleadoService, ReservaService, SalaService};
use std::sync::Arc;
use tower_http::services::ServeDir;

/// Crea el router con todas las rutas de la interfaz web
pub fn crear_router_web(
    empleado_service: Arc<dyn EmpleadoService>,
    reserva_service: Arc<dyn ReservaService>,
    sala_service: Arc<dyn SalaService>,
) -> Router {
    Router::new()
        // Página principal
        .route("/", get(handlers::index))
        // Rutas de empleados
        .route("/empleados", get(handlers::listar_empleados_page))
        .route("/empleados/nuevo", get(handlers::nuevo_empleado_form))
        .route("/empleados/crear", post(handlers::crear_empleado_submit))
        .route("/empleados/:id/activar", post(handlers::activar_empleado))
        .route(
            "/empleados/:id/desactivar",
            post(handlers::desactivar_empleado),
        )
        // Rutas de reservas
        .route("/reservas", get(handlers::listar_reservas_page))
        .route("/reservas/:id/confirmar", post(handlers::confirmar_reserva))
        .route("/reservas/:id/cancelar", post(handlers::cancelar_reserva))
        // Disponibilidad
        .route("/disponibilidad", get(handlers::disponibilidad_page))
        // Salas
        .route("/salas", get(handlers::listar_salas_page))
        // .route("/salas/:id/activar", post(handlers::activar_sala))
        // .route("/salas/:id/desactivar", post(handlers::desactivar_sala))
        // Archivos estáticos (CSS, imágenes, etc.)
        .nest_service("/static", ServeDir::new("crates/web-ui/static"))
        // Inyectar servicios
        .layer(axum::Extension(empleado_service))
        .layer(axum::Extension(reserva_service))
        .layer(axum::Extension(sala_service))
}
