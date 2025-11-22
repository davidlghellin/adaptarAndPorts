// ⚙️ APLICACIÓN - Implementa los CASOS DE USO
//
// Esta capa orquesta el dominio usando los puertos
// Es donde vive la lógica de aplicación (no confundir con lógica de negocio)
//
// Responsabilidades:
// - Coordinar operaciones
// - Validar permisos
// - Gestionar transacciones
// - Llamar al dominio

use reservas_domain::Reserva;
use reservas_ports::{ReservaRepository, ReservaService};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Servicio de aplicación que implementa los casos de uso
pub struct ReservaServiceImpl<R: ReservaRepository> {
    repository: R,
}

impl<R: ReservaRepository> ReservaServiceImpl<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: ReservaRepository + Send + Sync> ReservaService for ReservaServiceImpl<R> {
    async fn crear_reserva(
        &self,
        nombre_cliente: String,
        fecha: DateTime<Utc>,
        num_personas: u8,
    ) -> Result<Reserva, String> {
        // Generamos un ID único
        let id = Uuid::new_v4().to_string();

        // Creamos la entidad usando la lógica del dominio
        let reserva = Reserva::new(id.clone(), nombre_cliente, fecha, num_personas)
            .map_err(|e| format!("Error de validación: {:?}", e))?;

        // Verificamos que no exista
        if self.repository.existe(&id).await? {
            return Err("La reserva ya existe".to_string());
        }

        // Persistimos usando el puerto de salida
        self.repository.guardar(&reserva).await?;

        Ok(reserva)
    }

    async fn obtener_reserva(&self, id: &str) -> Result<Option<Reserva>, String> {
        self.repository.obtener(id).await
    }

    async fn listar_reservas(&self) -> Result<Vec<Reserva>, String> {
        self.repository.listar().await
    }

    async fn confirmar_reserva(&self, id: &str) -> Result<Reserva, String> {
        let mut reserva = self
            .repository
            .obtener(id)
            .await?
            .ok_or_else(|| "Reserva no encontrada".to_string())?;

        // Lógica de dominio
        reserva.confirmar();

        // Persistimos el cambio
        self.repository.actualizar(&reserva).await?;

        Ok(reserva)
    }

    async fn cancelar_reserva(&self, id: &str) -> Result<Reserva, String> {
        let mut reserva = self
            .repository
            .obtener(id)
            .await?
            .ok_or_else(|| "Reserva no encontrada".to_string())?;

        // Lógica de dominio
        reserva.cancelar();

        // Persistimos el cambio
        self.repository.actualizar(&reserva).await?;

        Ok(reserva)
    }
}
