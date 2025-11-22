// DTOs (Data Transfer Objects) para la API REST
// Estos objetos son la "frontera" entre el mundo HTTP/JSON y nuestro dominio

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ============= DTOs para Empleados =============

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CrearEmpleadoRequest {
    #[schema(example = "Juan López")]
    pub nombre: String,
    #[schema(example = "juan@empresa.com")]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EmpleadoResponse {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: String,
    #[schema(example = "Juan López")]
    pub nombre: String,
    #[schema(example = "juan@empresa.com")]
    pub email: String,
    pub activo: bool,
}

// ============= DTOs para Reservas =============

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CrearReservaRequest {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub empleado_id: String,
    /// Fecha y hora de inicio del slot en formato ISO 8601
    #[schema(example = "2025-11-25T10:00:00Z")]
    pub inicio_slot: DateTime<Utc>,
    #[schema(example = "Reunión con cliente importante")]
    pub descripcion: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReservaResponse {
    pub id: String,
    pub empleado_id: String,
    pub slot_inicio: DateTime<Utc>,
    pub slot_fin: DateTime<Utc>,
    pub descripcion: String,
    #[schema(example = "pendiente")]
    pub estado: String,
}

// ============= DTOs para Disponibilidad =============

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConsultarDisponibilidadRequest {
    /// Fecha en formato ISO 8601 (YYYY-MM-DD)
    #[schema(example = "2025-11-25")]
    pub fecha: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DisponibilidadEmpleadoResponse {
    pub empleado_id: String,
    pub empleado_nombre: String,
    pub slot_inicio: DateTime<Utc>,
    pub slot_fin: DateTime<Utc>,
    pub disponible: bool,
    pub reserva_id: Option<String>,
    pub descripcion_reserva: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TablaDisponibilidadResponse {
    #[schema(example = "2025-11-25")]
    pub fecha: String,
    pub slots: Vec<SlotInfo>,
    pub disponibilidad: Vec<DisponibilidadEmpleadoResponse>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SlotInfo {
    pub inicio: DateTime<Utc>,
    pub fin: DateTime<Utc>,
    pub hora: u32,
}

// ============= DTOs genéricos =============

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MensajeResponse {
    pub mensaje: String,
}
