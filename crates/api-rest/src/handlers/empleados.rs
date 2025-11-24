// Handlers para endpoints de Empleados
// Estos son ADAPTADORES DE ENTRADA que traducen HTTP -> Casos de Uso

use crate::dtos::{CrearEmpleadoRequest, EmpleadoResponse, ErrorResponse};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use reservas_ports::r#in::empleado_service::EmpleadoService;
use std::sync::Arc;

/// Crear un nuevo empleado
#[utoipa::path(
    post,
    path = "/empleados",
    request_body = CrearEmpleadoRequest,
    responses(
        (status = 201, description = "Empleado creado exitosamente", body = EmpleadoResponse),
        (status = 400, description = "Error de validación", body = ErrorResponse)
    ),
    tag = "Empleados"
)]
pub async fn crear_empleado(
    Extension(service): Extension<Arc<dyn EmpleadoService>>,
    Json(request): Json<CrearEmpleadoRequest>,
) -> Response {
    match service.crear_empleado(request.nombre, request.email).await {
        Ok(empleado) => {
            let response: EmpleadoResponse = empleado.into();
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: e })).into_response(),
    }
}

/// Listar todos los empleados
#[utoipa::path(
    get,
    path = "/empleados",
    responses(
        (status = 200, description = "Lista de empleados", body = Vec<EmpleadoResponse>),
        (status = 500, description = "Error interno", body = ErrorResponse)
    ),
    tag = "Empleados"
)]
pub async fn listar_empleados(Extension(service): Extension<Arc<dyn EmpleadoService>>) -> Response {
    match service.listar_empleados().await {
        Ok(empleados) => {
            let response: Vec<EmpleadoResponse> = empleados.into_iter().map(|e| e.into()).collect();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: e }),
        )
            .into_response(),
    }
}

/// Obtener un empleado por ID
#[utoipa::path(
    get,
    path = "/empleados/{id}",
    params(
        ("id" = String, Path, description = "ID del empleado")
    ),
    responses(
        (status = 200, description = "Empleado encontrado", body = EmpleadoResponse),
        (status = 404, description = "Empleado no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse)
    ),
    tag = "Empleados"
)]
pub async fn obtener_empleado(
    Extension(service): Extension<Arc<dyn EmpleadoService>>,
    Path(id): Path<String>,
) -> Response {
    match service.obtener_empleado(&id).await {
        Ok(Some(empleado)) => {
            let response: EmpleadoResponse = empleado.into();
            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Empleado {} no encontrado", id),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: e }),
        )
            .into_response(),
    }
}

/// Desactivar un empleado
#[utoipa::path(
    post,
    path = "/empleados/{id}/desactivar",
    params(
        ("id" = String, Path, description = "ID del empleado")
    ),
    responses(
        (status = 200, description = "Empleado desactivado", body = EmpleadoResponse),
        (status = 400, description = "Error de validación", body = ErrorResponse)
    ),
    tag = "Empleados"
)]
pub async fn desactivar_empleado(
    Extension(service): Extension<Arc<dyn EmpleadoService>>,
    Path(id): Path<String>,
) -> Response {
    match service.desactivar_empleado(&id).await {
        Ok(empleado) => {
            let response: EmpleadoResponse = empleado.into();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: e })).into_response(),
    }
}

/// Activar un empleado
#[utoipa::path(
    post,
    path = "/empleados/{id}/activar",
    params(
        ("id" = String, Path, description = "ID del empleado")
    ),
    responses(
        (status = 200, description = "Empleado activado", body = EmpleadoResponse),
        (status = 400, description = "Error de validación", body = ErrorResponse)
    ),
    tag = "Empleados"
)]
pub async fn activar_empleado(
    Extension(service): Extension<Arc<dyn EmpleadoService>>,
    Path(id): Path<String>,
) -> Response {
    match service.activar_empleado(&id).await {
        Ok(empleado) => {
            let response: EmpleadoResponse = empleado.into();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: e })).into_response(),
    }
}
