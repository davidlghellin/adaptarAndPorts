use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============= DTOs (deben coincidir con la API) =============

// 👇 request -> Serialize

#[derive(Debug, Serialize)]
pub struct CrearEmpleadoRequest {
    pub nombre: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct EmpleadoResponse {
    pub id: String,
    pub nombre: String,
    pub email: String,
    pub activo: bool,
}

#[derive(Debug, Serialize)]
pub struct CrearReservaRequest {
    pub empleado_id: String,
    pub inicio_slot: DateTime<Utc>,
    pub descripcion: String,
}

#[derive(Debug, Deserialize)]
pub struct ReservaResponse {
    pub id: String,
    pub empleado_id: String,
    pub slot_inicio: DateTime<Utc>,
    pub slot_fin: DateTime<Utc>,
    pub descripcion: String,
    pub estado: String,
}

#[derive(Debug, Deserialize)]
pub struct TablaDisponibilidadResponse {
    pub fecha: String,
    pub slots: Vec<SlotInfo>,
    pub disponibilidad: Vec<DisponibilidadEmpleadoResponse>,
}

#[derive(Debug, Serialize)]
pub struct CrearSalaRequest {
    pub nombre: String,
    pub capacidad: u32,
}

#[derive(Debug, Deserialize)]
pub struct SalaResponse {
    pub id: String,
    pub nombre: String,
    pub capacidad: u32,
    pub activa: bool,
}

#[derive(Debug, Deserialize)]
pub struct SlotInfo {
    pub inicio: DateTime<Utc>,
    pub fin: DateTime<Utc>,
    pub hora: u32,
}

#[derive(Debug, Deserialize)]
pub struct DisponibilidadEmpleadoResponse {
    pub empleado_id: String,
    pub empleado_nombre: String,
    pub slot_inicio: DateTime<Utc>,
    pub slot_fin: DateTime<Utc>,
    pub disponible: bool,
    pub reserva_id: Option<String>,
    pub descripcion_reserva: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
