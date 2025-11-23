// Definición de templates usando Askama

use askama::Template;
use chrono::{DateTime, Utc};

// ============= DTOs para templates =============

#[derive(Debug)]
pub struct EmpleadoView {
    pub id: String,
    pub nombre: String,
    pub email: String,
    pub activo: bool,
}

#[derive(Debug)]
pub struct ReservaView {
    pub id: String,
    pub empleado_id: String,
    pub slot_inicio: DateTime<Utc>,
    pub slot_fin: DateTime<Utc>,
    pub descripcion: String,
    pub estado: String,
}

// ============= Templates =============

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "empleados.html")]
pub struct EmpleadosTemplate {
    pub empleados: Vec<EmpleadoView>,
}

#[derive(Template)]
#[template(path = "empleado_form.html")]
pub struct EmpleadoFormTemplate;

#[derive(Template)]
#[template(path = "reservas.html")]
pub struct ReservasTemplate {
    pub reservas: Vec<ReservaView>,
}
