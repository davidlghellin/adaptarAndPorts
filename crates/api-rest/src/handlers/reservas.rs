// Handlers para endpoints de Reservas

use crate::dtos::{CrearReservaRequest, ErrorResponse, ReservaResponse};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use reservas_domain::Slot;
use reservas_ports::r#in::reserva_service::ReservaService;

use std::sync::Arc;

/// Crear una nueva reserva
#[utoipa::path(
    post,
    path = "/reservas",
    request_body = CrearReservaRequest,
    responses(
        (status = 201, description = "Reserva creada exitosamente", body = ReservaResponse),
        (status = 400, description = "Error de validación o slot ocupado", body = ErrorResponse)
    ),
    tag = "Reservas"
)]
pub async fn crear_reserva(
    Extension(service): Extension<Arc<dyn ReservaService>>,
    Json(request): Json<CrearReservaRequest>,
) -> Response {
    // Convertir DateTime a Slot
    let slot = Slot::new(request.inicio_slot);

    match service
        .crear_reserva(request.empleado_id, slot, request.descripcion)
        .await
    {
        Ok(reserva) => {
            let response: ReservaResponse = reserva.into();
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: e })).into_response(),
    }
}

/// Listar todas las reservas
#[utoipa::path(
    get,
    path = "/reservas",
    responses(
        (status = 200, description = "Lista de reservas", body = Vec<ReservaResponse>),
        (status = 500, description = "Error interno", body = ErrorResponse)
    ),
    tag = "Reservas"
)]
pub async fn listar_reservas(Extension(service): Extension<Arc<dyn ReservaService>>) -> Response {
    match service.listar_reservas().await {
        Ok(reservas) => {
            let response: Vec<ReservaResponse> = reservas.into_iter().map(|r| r.into()).collect();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: e }),
        )
            .into_response(),
    }
}

/// Obtener una reserva por ID
#[utoipa::path(
    get,
    path = "/reservas/{id}",
    params(
        ("id" = String, Path, description = "ID de la reserva")
    ),
    responses(
        (status = 200, description = "Reserva encontrada", body = ReservaResponse),
        (status = 404, description = "Reserva no encontrada", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse)
    ),
    tag = "Reservas"
)]
pub async fn obtener_reserva(
    Extension(service): Extension<Arc<dyn ReservaService>>,
    Path(id): Path<String>,
) -> Response {
    match service.obtener_reserva(&id).await {
        Ok(Some(reserva)) => {
            let response: ReservaResponse = reserva.into();
            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Reserva {} no encontrada", id),
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

/// Listar reservas de un empleado
#[utoipa::path(
    get,
    path = "/empleados/{id}/reservas",
    params(
        ("id" = String, Path, description = "ID del empleado")
    ),
    responses(
        (status = 200, description = "Lista de reservas del empleado", body = Vec<ReservaResponse>),
        (status = 500, description = "Error interno", body = ErrorResponse)
    ),
    tag = "Reservas"
)]
pub async fn listar_reservas_empleado(
    Extension(service): Extension<Arc<dyn ReservaService>>,
    Path(empleado_id): Path<String>,
) -> Response {
    match service.listar_reservas_empleado(&empleado_id).await {
        Ok(reservas) => {
            let response: Vec<ReservaResponse> = reservas.into_iter().map(|r| r.into()).collect();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: e }),
        )
            .into_response(),
    }
}

/// Confirmar una reserva
#[utoipa::path(
    post,
    path = "/reservas/{id}/confirmar",
    params(
        ("id" = String, Path, description = "ID de la reserva")
    ),
    responses(
        (status = 200, description = "Reserva confirmada", body = ReservaResponse),
        (status = 400, description = "Error de validación", body = ErrorResponse)
    ),
    tag = "Reservas"
)]
pub async fn confirmar_reserva(
    Extension(service): Extension<Arc<dyn ReservaService>>,
    Path(id): Path<String>,
) -> Response {
    match service.confirmar_reserva(&id).await {
        Ok(reserva) => {
            let response: ReservaResponse = reserva.into();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: e })).into_response(),
    }
}

/// Cancelar una reserva
#[utoipa::path(
    post,
    path = "/reservas/{id}/cancelar",
    params(
        ("id" = String, Path, description = "ID de la reserva")
    ),
    responses(
        (status = 200, description = "Reserva cancelada", body = ReservaResponse),
        (status = 400, description = "Error de validación", body = ErrorResponse)
    ),
    tag = "Reservas"
)]
pub async fn cancelar_reserva(
    Extension(service): Extension<Arc<dyn ReservaService>>,
    Path(id): Path<String>,
) -> Response {
    match service.cancelar_reserva(&id).await {
        Ok(reserva) => {
            let response: ReservaResponse = reserva.into();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: e })).into_response(),
    }
}
