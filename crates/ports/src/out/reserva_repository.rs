use async_trait::async_trait;
use reservas_domain::{Reserva, Slot};

/// Puerto de salida (OUTPUT PORT): Define cómo persistir reservas
#[async_trait]
pub trait ReservaRepository {
    async fn guardar(&self, reserva: &Reserva) -> Result<(), String>;

    async fn obtener(&self, id: &str) -> Result<Option<Reserva>, String>;

    async fn listar(&self) -> Result<Vec<Reserva>, String>;

    async fn listar_por_empleado(&self, empleado_id: &str) -> Result<Vec<Reserva>, String>;

    async fn listar_por_slot(&self, slot: &Slot) -> Result<Vec<Reserva>, String>;

    async fn actualizar(&self, reserva: &Reserva) -> Result<(), String>;

    async fn existe(&self, id: &str) -> Result<bool, String>;

    async fn existe_para_empleado_en_slot(
        &self,
        empleado_id: &str,
        slot: &Slot,
    ) -> Result<bool, String>;
}
