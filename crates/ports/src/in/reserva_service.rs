use async_trait::async_trait;
use reservas_domain::{Reserva, Slot};

/// Puerto de entrada (INPUT PORT): Define cómo usar el sistema
/// Este es el "caso de uso" de nuestro sistema
#[async_trait]
pub trait ReservaService: Send + Sync {
    /// Crea una nueva reserva para un empleado en un slot específico
    async fn crear_reserva(
        &self,
        empleado_id: String,
        slot: Slot,
        descripcion: String,
    ) -> Result<Reserva, String>;

    /// Obtiene una reserva por su ID
    async fn obtener_reserva(&self, id: &str) -> Result<Option<Reserva>, String>;

    /// Lista todas las reservas activas
    async fn listar_reservas(&self) -> Result<Vec<Reserva>, String>;

    /// Lista las reservas de un empleado específico
    async fn listar_reservas_empleado(&self, empleado_id: &str) -> Result<Vec<Reserva>, String>;

    /// Confirma una reserva
    async fn confirmar_reserva(&self, id: &str) -> Result<Reserva, String>;

    /// Cancela una reserva
    async fn cancelar_reserva(&self, id: &str) -> Result<Reserva, String>;
}
