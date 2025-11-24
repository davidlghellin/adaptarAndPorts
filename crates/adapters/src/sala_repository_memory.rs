use async_trait::async_trait;
use reservas_domain::Sala;
use reservas_ports::out::sala_repository::SalaRepository;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct InMemorySalaRepository {
    salas: Arc<RwLock<HashMap<String, Sala>>>,
}

impl Default for InMemorySalaRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemorySalaRepository {
    pub fn new() -> Self {
        Self {
            salas: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl SalaRepository for InMemorySalaRepository {
    async fn guardar(&self, sala: &Sala) -> Result<(), String> {
        let mut salas = self.salas.write().await;
        salas.insert(sala.id.clone(), sala.clone());
        Ok(())
    }

    async fn obtener(&self, id: &str) -> Result<Option<Sala>, String> {
        let salas = self.salas.read().await;
        Ok(salas.get(id).cloned())
    }

    async fn listar(&self) -> Result<Vec<Sala>, String> {
        let salas = self.salas.read().await;
        Ok(salas.values().cloned().collect())
    }

    async fn actualizar(&self, sala: &Sala) -> Result<(), String> {
        let mut salas = self.salas.write().await;
        if salas.contains_key(&sala.id) {
            salas.insert(sala.id.clone(), sala.clone());
            Ok(())
        } else {
            Err("Sala no encontrada".to_string())
        }
    }
}
