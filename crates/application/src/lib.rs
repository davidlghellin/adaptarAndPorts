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

use async_trait::async_trait;
use reservas_domain::{Empleado, Reserva, Slot};
use reservas_ports::{EmpleadoRepository, EmpleadoService, ReservaRepository, ReservaService};
use uuid::Uuid;

/// Servicio de aplicación que implementa los casos de uso de reservas
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
        empleado_id: String,
        slot: Slot,
        descripcion: String,
    ) -> Result<Reserva, String> {
        // Generamos un ID único
        let id = Uuid::new_v4().to_string();

        // Verificamos que el empleado no tenga ya una reserva en este slot
        if self
            .repository
            .existe_para_empleado_en_slot(&empleado_id, &slot)
            .await?
        {
            return Err(format!(
                "El empleado {} ya tiene una reserva en el slot {}",
                empleado_id,
                slot.formato_legible()
            ));
        }

        // Creamos la entidad usando la lógica del dominio
        let reserva = Reserva::new(id, empleado_id, slot, descripcion)
            .map_err(|e| format!("Error de validación: {:?}", e))?;

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

    async fn listar_reservas_empleado(&self, empleado_id: &str) -> Result<Vec<Reserva>, String> {
        self.repository.listar_por_empleado(empleado_id).await
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

/// Servicio de aplicación para gestión de empleados
pub struct EmpleadoServiceImpl<R: EmpleadoRepository> {
    repository: R,
}

impl<R: EmpleadoRepository> EmpleadoServiceImpl<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: EmpleadoRepository + Send + Sync> EmpleadoService for EmpleadoServiceImpl<R> {
    async fn crear_empleado(&self, nombre: String, email: String) -> Result<Empleado, String> {
        let id = Uuid::new_v4().to_string();
        let empleado = Empleado::new(id, nombre, email);

        self.repository.guardar(&empleado).await?;

        Ok(empleado)
    }

    async fn obtener_empleado(&self, id: &str) -> Result<Option<Empleado>, String> {
        self.repository.obtener(id).await
    }

    async fn listar_empleados(&self) -> Result<Vec<Empleado>, String> {
        self.repository.listar().await
    }

    async fn desactivar_empleado(&self, id: &str) -> Result<Empleado, String> {
        let mut empleado = self
            .repository
            .obtener(id)
            .await?
            .ok_or_else(|| "Empleado no encontrado".to_string())?;

        empleado.desactivar();

        self.repository.actualizar(&empleado).await?;

        Ok(empleado)
    }
}
