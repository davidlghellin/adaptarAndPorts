// Cliente HTTP para interactuar con la API REST

use crate::response::{
    CrearEmpleadoRequest, CrearReservaRequest, CrearSalaRequest, EmpleadoResponse, ErrorResponse,
    ReservaResponse, SalaResponse, TablaDisponibilidadResponse,
};
use chrono::{DateTime, Utc};

// ============= Cliente API =============

pub struct ApiClient {
    base_url: String,
    client: reqwest::blocking::Client,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::blocking::Client::new(),
        }
    }

    // Empleados

    pub fn crear_empleado(
        &self,
        nombre: String,
        email: String,
    ) -> Result<EmpleadoResponse, String> {
        let request = CrearEmpleadoRequest { nombre, email };

        let response = self
            .client
            .post(format!("{}/empleados", self.base_url))
            .json(&request)
            .send()
            .map_err(|e| format!("Error de conexión: {}", e))?;

        if response.status().is_success() {
            response
                .json()
                .map_err(|e| format!("Error parseando respuesta: {}", e))
        } else {
            let error: ErrorResponse = response
                .json()
                .map_err(|e| format!("Error parseando error: {}", e))?;
            Err(error.error)
        }
    }

    pub fn listar_empleados(&self) -> Result<Vec<EmpleadoResponse>, String> {
        let response = self
            .client
            .get(format!("{}/empleados", self.base_url))
            .send()
            .map_err(|e| format!("Error de conexión: {}", e))?;

        if response.status().is_success() {
            response
                .json()
                .map_err(|e| format!("Error parseando respuesta: {}", e))
        } else {
            Err("Error obteniendo empleados".to_string())
        }
    }

    pub fn obtener_empleado(&self, id: &str) -> Result<EmpleadoResponse, String> {
        let response = self
            .client
            .get(format!("{}/empleados/{}", self.base_url, id))
            .send()
            .map_err(|e| format!("Error de conexión: {}", e))?;

        if response.status().is_success() {
            response
                .json()
                .map_err(|e| format!("Error parseando respuesta: {}", e))
        } else {
            let error: ErrorResponse = response
                .json()
                .map_err(|e| format!("Error parseando error: {}", e))?;
            Err(error.error)
        }
    }

    pub fn activar_empleado(&self, id: &str) -> Result<EmpleadoResponse, String> {
        let response = self
            .client
            .post(format!("{}/empleados/{}/activar", self.base_url, id))
            .send()
            .map_err(|e| format!("Error de conexión: {}", e))?;

        if response.status().is_success() {
            response
                .json()
                .map_err(|e| format!("Error parseando respuesta: {}", e))
        } else {
            let error: ErrorResponse = response
                .json()
                .map_err(|e| format!("Error parseando error: {}", e))?;
            Err(error.error)
        }
    }

    pub fn desactivar_empleado(&self, id: &str) -> Result<EmpleadoResponse, String> {
        let response = self
            .client
            .post(format!("{}/empleados/{}/desactivar", self.base_url, id))
            .send()
            .map_err(|e| format!("Error de conexión: {}", e))?;

        if response.status().is_success() {
            response
                .json()
                .map_err(|e| format!("Error parseando respuesta: {}", e))
        } else {
            let error: ErrorResponse = response
                .json()
                .map_err(|e| format!("Error parseando error: {}", e))?;
            Err(error.error)
        }
    }

    // Reservas

    pub fn crear_reserva(
        &self,
        empleado_id: String,
        inicio_slot: DateTime<Utc>,
        descripcion: String,
    ) -> Result<ReservaResponse, String> {
        let request = CrearReservaRequest {
            empleado_id,
            inicio_slot,
            descripcion,
        };

        let response = self
            .client
            .post(format!("{}/reservas", self.base_url))
            .json(&request)
            .send()
            .map_err(|e| format!("Error de conexión: {}", e))?;

        if response.status().is_success() {
            response
                .json()
                .map_err(|e| format!("Error parseando respuesta: {}", e))
        } else {
            let error: ErrorResponse = response
                .json()
                .map_err(|e| format!("Error parseando error: {}", e))?;
            Err(error.error)
        }
    }

    pub fn listar_reservas(&self) -> Result<Vec<ReservaResponse>, String> {
        let response = self
            .client
            .get(format!("{}/reservas", self.base_url))
            .send()
            .map_err(|e| format!("Error de conexión: {}", e))?;

        if response.status().is_success() {
            response
                .json()
                .map_err(|e| format!("Error parseando respuesta: {}", e))
        } else {
            Err("Error obteniendo reservas".to_string())
        }
    }

    pub fn listar_reservas_empleado(
        &self,
        empleado_id: &str,
    ) -> Result<Vec<ReservaResponse>, String> {
        let response = self
            .client
            .get(format!(
                "{}/empleados/{}/reservas",
                self.base_url, empleado_id
            ))
            .send()
            .map_err(|e| format!("Error de conexión: {}", e))?;

        if response.status().is_success() {
            response
                .json()
                .map_err(|e| format!("Error parseando respuesta: {}", e))
        } else {
            Err("Error obteniendo reservas del empleado".to_string())
        }
    }

    pub fn confirmar_reserva(&self, id: &str) -> Result<ReservaResponse, String> {
        let response = self
            .client
            .post(format!("{}/reservas/{}/confirmar", self.base_url, id))
            .send()
            .map_err(|e| format!("Error de conexión: {}", e))?;

        if response.status().is_success() {
            response
                .json()
                .map_err(|e| format!("Error parseando respuesta: {}", e))
        } else {
            let error: ErrorResponse = response
                .json()
                .map_err(|e| format!("Error parseando error: {}", e))?;
            Err(error.error)
        }
    }

    pub fn cancelar_reserva(&self, id: &str) -> Result<ReservaResponse, String> {
        let response = self
            .client
            .post(format!("{}/reservas/{}/cancelar", self.base_url, id))
            .send()
            .map_err(|e| format!("Error de conexión: {}", e))?;

        if response.status().is_success() {
            response
                .json()
                .map_err(|e| format!("Error parseando respuesta: {}", e))
        } else {
            let error: ErrorResponse = response
                .json()
                .map_err(|e| format!("Error parseando error: {}", e))?;
            Err(error.error)
        }
    }

    // Disponibilidad

    pub fn obtener_disponibilidad(
        &self,
        fecha: &str,
    ) -> Result<TablaDisponibilidadResponse, String> {
        let response = self
            .client
            .get(format!("{}/disponibilidad?fecha={}", self.base_url, fecha))
            .send()
            .map_err(|e| format!("Error de conexión: {}", e))?;

        if response.status().is_success() {
            response
                .json()
                .map_err(|e| format!("Error parseando respuesta: {}", e))
        } else {
            let error: ErrorResponse = response
                .json()
                .map_err(|e| format!("Error parseando error: {}", e))?;
            Err(error.error)
        }
    }

    // Salas
    pub fn listar_salas(&self) -> Result<Vec<SalaResponse>, String> {
        let response = self
            .client
            .get(format!("{}/salas", self.base_url))
            .send()
            .map_err(|e| format!("Error de conexión: {}", e))?;

        if response.status().is_success() {
            response
                .json()
                .map_err(|e| format!("Error parseando respuesta: {}", e))
        } else {
            Err("Error obteniendo salas".to_string())
        }
    }
    // Salas

    pub fn crear_sala(&self, nombre: String, capacidad: u32) -> Result<SalaResponse, String> {
        let request = CrearSalaRequest { nombre, capacidad };

        let response = self
            .client
            .post(format!("{}/salas", self.base_url))
            .json(&request)
            .send()
            .map_err(|e| format!("Error de conexión: {}", e))?;

        if response.status().is_success() {
            response
                .json()
                .map_err(|e| format!("Error parseando respuesta: {}", e))
        } else {
            let error: ErrorResponse = response
                .json()
                .map_err(|e| format!("Error parseando error: {}", e))?;
            Err(error.error)
        }
    }
}
