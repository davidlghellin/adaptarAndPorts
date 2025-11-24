// Handlers HTTP para la interfaz web

use askama_axum::IntoResponse;
use axum::{
    extract::{Form, Path},
    http::StatusCode,
    response::Redirect,
    Extension,
};
use reservas_domain::reserva::EstadoReserva;
use reservas_ports::r#in::reserva_service::ReservaService;
use serde::Deserialize;
use std::sync::Arc;

use reservas_ports::r#in::empleado_service::EmpleadoService;
use reservas_ports::r#in::sala_service::SalaService;

use crate::templates::*;

// ============= Handlers de páginas =============

pub async fn index() -> impl IntoResponse {
    IndexTemplate
}

pub async fn listar_empleados_page(
    Extension(service): Extension<Arc<dyn EmpleadoService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let empleados = service
        .listar_empleados()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let empleados_view: Vec<EmpleadoView> = empleados
        .into_iter()
        .map(|e| EmpleadoView {
            id: e.id,
            nombre: e.nombre,
            email: e.email,
            activo: e.activo,
        })
        .collect();

    Ok(EmpleadosTemplate {
        empleados: empleados_view,
    })
}

pub async fn nuevo_empleado_form() -> impl IntoResponse {
    EmpleadoFormTemplate
}

pub async fn nuevo_sala_form() -> impl IntoResponse {
    SalaFormTemplate
}

#[derive(Deserialize)]
pub struct CrearEmpleadoForm {
    nombre: String,
    email: String,
}

#[derive(Deserialize)]
pub struct CrearSalaForm {
    nombre: String,
    capacidad: u32,
}

pub async fn crear_empleado_submit(
    Extension(service): Extension<Arc<dyn EmpleadoService>>,
    Form(form): Form<CrearEmpleadoForm>,
) -> Result<Redirect, StatusCode> {
    service
        .crear_empleado(form.nombre, form.email)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Redirect::to("/empleados"))
}

pub async fn crear_sala_submit(
    Extension(service): Extension<Arc<dyn SalaService>>,
    Form(form): Form<CrearSalaForm>,
) -> Result<Redirect, StatusCode> {
    service
        .crear_sala(form.nombre, form.capacidad)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Redirect::to("/salas"))
}

pub async fn activar_empleado(
    Extension(service): Extension<Arc<dyn EmpleadoService>>,
    Path(id): Path<String>,
) -> Result<Redirect, StatusCode> {
    service
        .activar_empleado(&id)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Redirect::to("/empleados"))
}

pub async fn desactivar_empleado(
    Extension(service): Extension<Arc<dyn EmpleadoService>>,
    Path(id): Path<String>,
) -> Result<Redirect, StatusCode> {
    service
        .desactivar_empleado(&id)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Redirect::to("/empleados"))
}

pub async fn listar_reservas_page(
    Extension(service): Extension<Arc<dyn ReservaService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let reservas = service
        .listar_reservas()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let reservas_view: Vec<ReservaView> = reservas
        .into_iter()
        .map(|r| {
            let estado_str = match r.estado {
                EstadoReserva::Pendiente => "pendiente",
                EstadoReserva::Confirmada => "confirmada",
                EstadoReserva::Cancelada => "cancelada",
            };
            ReservaView {
                id: r.id,
                empleado_id: r.empleado_id,
                slot_inicio: r.slot.inicio,
                slot_fin: r.slot.fin(),
                descripcion: r.descripcion,
                estado: estado_str.to_string(),
            }
        })
        .collect();

    Ok(ReservasTemplate {
        reservas: reservas_view,
    })
}

pub async fn confirmar_reserva(
    Extension(service): Extension<Arc<dyn ReservaService>>,
    Path(id): Path<String>,
) -> Result<Redirect, StatusCode> {
    service
        .confirmar_reserva(&id)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Redirect::to("/reservas"))
}

pub async fn cancelar_reserva(
    Extension(service): Extension<Arc<dyn ReservaService>>,
    Path(id): Path<String>,
) -> Result<Redirect, StatusCode> {
    service
        .cancelar_reserva(&id)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Redirect::to("/reservas"))
}

pub async fn disponibilidad_page() -> impl IntoResponse {
    DisponibilidadTemplate
}

pub async fn listar_salas_page(
    Extension(service): Extension<Arc<dyn SalaService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let salas = service
        .listar_salas()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let salas_view: Vec<SalaView> = salas
        .into_iter()
        .map(|s| SalaView {
            id: s.id,
            nombre: s.nombre,
            capacidad: s.capacidad,
            activa: s.activa,
        })
        .collect();

    Ok(SalasTemplate { salas: salas_view })
}

pub async fn activar_sala(
    Extension(service): Extension<Arc<dyn SalaService>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    service
        .activar_sala(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/salas"))
}

pub async fn desactivar_sala(
    Extension(service): Extension<Arc<dyn SalaService>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    service
        .desactivar_sala(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/salas"))
}
