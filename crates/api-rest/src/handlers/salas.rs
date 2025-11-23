use crate::dtos::{CrearSalaRequest, ErrorResponse, SalaResponse};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use reservas_ports::SalaService;
use std::sync::Arc;

/// Listar todas las salas
#[utoipa::path(
    get,
    path = "/salas",
    responses(
        (status = 200, description = "Lista de salas", body = [SalaResponse])
    ),
    tag = "Salas"
)]
pub async fn listar_salas(Extension(service): Extension<Arc<dyn SalaService>>) -> Response {
    match service.listar_salas().await {
        Ok(salas) => {
            let response: Vec<SalaResponse> = salas.into_iter().map(|e| e.into()).collect();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: e }),
        )
            .into_response(),
    }
}

/// Crear nueva sala
#[utoipa::path(
    post,
    path = "/salas",
    request_body = CrearSalaRequest,
    responses(
        (status = 201, description = "Sala creada", body = SalaResponse)
    ),
    tag = "Salas"
)]
pub async fn crear_sala(
    Extension(service): Extension<Arc<dyn SalaService>>,
    Json(request): Json<CrearSalaRequest>,
) -> Result<(StatusCode, Json<SalaResponse>), StatusCode> {
    let sala = service
        .crear_sala(request.nombre, request.capacidad)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let response = SalaResponse {
        id: sala.id,
        nombre: sala.nombre,
        capacidad: sala.capacidad,
        activa: sala.activa,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
