// Adaptador de salida: Repositorio en memoria
// Implementa el puerto ReservaRepository usando un HashMap

use reservas_domain::Reserva;
use reservas_ports::ReservaRepository;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Implementación en memoria del repositorio
/// Perfecto para pruebas y desarrollo inicial
pub struct InMemoryReservaRepository {
    storage: Arc<RwLock<HashMap<String, Reserva>>>,
}

impl InMemoryReservaRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl ReservaRepository for InMemoryReservaRepository {
    async fn guardar(&self, reserva: &Reserva) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        storage.insert(reserva.id.clone(), reserva.clone());
        Ok(())
    }

    async fn obtener(&self, id: &str) -> Result<Option<Reserva>, String> {
        let storage = self.storage.read().await;
        Ok(storage.get(id).cloned())
    }

    async fn listar(&self) -> Result<Vec<Reserva>, String> {
        let storage = self.storage.read().await;
        Ok(storage.values().cloned().collect())
    }

    async fn actualizar(&self, reserva: &Reserva) -> Result<(), String> {
        let mut storage = self.storage.write().await;

        if !storage.contains_key(&reserva.id) {
            return Err("Reserva no encontrada".to_string());
        }

        storage.insert(reserva.id.clone(), reserva.clone());
        Ok(())
    }

    async fn existe(&self, id: &str) -> Result<bool, String> {
        let storage = self.storage.read().await;
        Ok(storage.contains_key(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reservas_domain::EstadoReserva;
    use chrono::Utc;

    #[tokio::test]
    async fn test_guardar_y_obtener() {
        let repo = InMemoryReservaRepository::new();
        let fecha = Utc::now() + chrono::Duration::days(1);

        let reserva = Reserva::new("1".to_string(), "Test".to_string(), fecha, 2).unwrap();

        repo.guardar(&reserva).await.unwrap();
        let obtenida = repo.obtener("1").await.unwrap();

        assert_eq!(obtenida, Some(reserva));
    }

    #[tokio::test]
    async fn test_actualizar() {
        let repo = InMemoryReservaRepository::new();
        let fecha = Utc::now() + chrono::Duration::days(1);

        let mut reserva = Reserva::new("1".to_string(), "Test".to_string(), fecha, 2).unwrap();
        repo.guardar(&reserva).await.unwrap();

        reserva.confirmar();
        repo.actualizar(&reserva).await.unwrap();

        let obtenida = repo.obtener("1").await.unwrap().unwrap();
        assert_eq!(obtenida.estado, EstadoReserva::Confirmada);
    }
}
