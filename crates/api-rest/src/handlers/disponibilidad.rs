// Handlers para endpoints de Disponibilidad

use crate::dtos::{
    DisponibilidadEmpleadoResponse, ErrorResponse, SlotInfo, TablaDisponibilidadResponse,
};
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use chrono::{Datelike, NaiveDate};
use reservas_domain::{DisponibilidadService, Slot};
use reservas_ports::{EmpleadoService, ReservaService};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct DisponibilidadQuery {
    /// Fecha en formato YYYY-MM-DD
    #[param(example = "2025-11-25")]
    pub fecha: String,
}

/// Obtener tabla de disponibilidad para una fecha
#[utoipa::path(
    get,
    path = "/disponibilidad",
    params(
        DisponibilidadQuery
    ),
    responses(
        (status = 200, description = "Tabla de disponibilidad", body = TablaDisponibilidadResponse),
        (status = 400, description = "Formato de fecha inválido", body = ErrorResponse),
        (status = 500, description = "Error interno", body = ErrorResponse)
    ),
    tag = "Disponibilidad"
)]
pub async fn obtener_disponibilidad(
    Extension(empleado_service): Extension<Arc<dyn EmpleadoService>>,
    Extension(reserva_service): Extension<Arc<dyn ReservaService>>,
    Query(params): Query<DisponibilidadQuery>,
) -> Response {
    // Parsear fecha
    let fecha = match NaiveDate::parse_from_str(&params.fecha, "%Y-%m-%d") {
        Ok(f) => f,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Formato de fecha inválido. Use YYYY-MM-DD".to_string(),
                }),
            )
                .into_response()
        }
    };

    // Obtener empleados y reservas
    let empleados = match empleado_service.listar_empleados().await {
        Ok(e) => e,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: err }),
            )
                .into_response()
        }
    };

    let reservas = match reserva_service.listar_reservas().await {
        Ok(r) => r,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: err }),
            )
                .into_response()
        }
    };

    // Generar slots del día
    let slots: Vec<Slot> = (9..18)
        .filter_map(|h| Slot::from_date_and_hour(fecha.year(), fecha.month(), fecha.day(), h))
        .collect();

    // Generar tabla de disponibilidad usando el servicio de dominio
    let tabla = DisponibilidadService::generar_tabla_disponibilidad(&empleados, &slots, &reservas);

    // Convertir a DTOs
    let slots_info: Vec<SlotInfo> = tabla.slots.iter().map(|s| s.into()).collect();

    let disponibilidad: Vec<DisponibilidadEmpleadoResponse> = tabla
        .disponibilidad
        .into_iter()
        .map(|d| DisponibilidadEmpleadoResponse {
            empleado_id: d.empleado_id,
            empleado_nombre: d.empleado_nombre,
            slot_inicio: d.slot.inicio,
            slot_fin: d.slot.fin(),
            disponible: d.disponible,
            reserva_id: d.reserva_id,
            descripcion_reserva: d.descripcion_reserva,
        })
        .collect();

    let response = TablaDisponibilidadResponse {
        fecha: params.fecha,
        slots: slots_info,
        disponibilidad,
    };

    (StatusCode::OK, Json(response)).into_response()
}
