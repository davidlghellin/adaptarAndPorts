use async_trait::async_trait;
use reservas_domain::Sala;

#[async_trait]
pub trait SalaRepository {
    async fn guardar(&self, sala: &Sala) -> Result<(), String>;
    async fn obtener(&self, id: &str) -> Result<Option<Sala>, String>;
    async fn listar(&self) -> Result<Vec<Sala>, String>;
    async fn actualizar(&self, sala: &Sala) -> Result<(), String>;
}
