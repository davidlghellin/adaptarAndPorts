use async_trait::async_trait;
use reservas_domain::Sala;

#[async_trait]
pub trait SalaService: Send + Sync {
    async fn crear_sala(&self, nombre: String, capacidad: u32) -> Result<Sala, String>;
    async fn listar_salas(&self) -> Result<Vec<Sala>, String>;
    async fn obtener_sala(&self, id: &str) -> Result<Option<Sala>, String>;
    async fn activar_sala(&self, id: &str) -> Result<(), String>;
    async fn desactivar_sala(&self, id: &str) -> Result<(), String>;
}
