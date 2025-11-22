// Adaptador de salida: Repositorio en memoria
// Implementa el puerto ReservaRepository usando un HashMap

use async_trait::async_trait;
use reservas_domain::{Reserva, Slot};
use reservas_ports::ReservaRepository;
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

    async fn listar_por_empleado(&self, empleado_id: &str) -> Result<Vec<Reserva>, String> {
        let storage = self.storage.read().await;
        Ok(storage
            .values()
            .filter(|r| r.empleado_id == empleado_id && r.esta_activa())
            .cloned()
            .collect())
    }

    async fn listar_por_slot(&self, slot: &Slot) -> Result<Vec<Reserva>, String> {
        let storage = self.storage.read().await;
        Ok(storage
            .values()
            .filter(|r| r.slot == *slot && r.esta_activa())
            .cloned()
            .collect())
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

    async fn existe_para_empleado_en_slot(
        &self,
        empleado_id: &str,
        slot: &Slot,
    ) -> Result<bool, String> {
        let storage = self.storage.read().await;
        Ok(storage
            .values()
            .any(|r| r.empleado_id == empleado_id && r.slot == *slot && r.esta_activa()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Utc};
    use reservas_domain::{EstadoReserva, Slot};

    #[tokio::test]
    async fn test_guardar_y_obtener() {
        let repo = InMemoryReservaRepository::new();
        let manyana = Utc::now() + chrono::Duration::days(1);
        let slot =
            Slot::from_date_and_hour(manyana.year(), manyana.month(), manyana.day(), 10).unwrap();

        let reserva = Reserva::new(
            "1".to_string(),
            "emp-001".to_string(),
            slot,
            "Test".to_string(),
        )
        .unwrap();

        repo.guardar(&reserva).await.unwrap();
        let obtenida = repo.obtener("1").await.unwrap();

        assert_eq!(obtenida, Some(reserva));
    }

    #[tokio::test]
    async fn test_actualizar() {
        let repo = InMemoryReservaRepository::new();
        let manyana = Utc::now() + chrono::Duration::days(1);
        let slot =
            Slot::from_date_and_hour(manyana.year(), manyana.month(), manyana.day(), 10).unwrap();

        let mut reserva = Reserva::new(
            "1".to_string(),
            "emp-001".to_string(),
            slot,
            "Test".to_string(),
        )
        .unwrap();
        repo.guardar(&reserva).await.unwrap();

        reserva.confirmar();
        repo.actualizar(&reserva).await.unwrap();

        let obtenida = repo.obtener("1").await.unwrap().unwrap();
        assert_eq!(obtenida.estado, EstadoReserva::Confirmada);
    }

    #[tokio::test]
    async fn test_existe_para_empleado_en_slot() {
        let repo = InMemoryReservaRepository::new();
        let manyana = Utc::now() + chrono::Duration::days(1);
        let slot =
            Slot::from_date_and_hour(manyana.year(), manyana.month(), manyana.day(), 10).unwrap();

        let reserva = Reserva::new(
            "1".to_string(),
            "emp-001".to_string(),
            slot.clone(),
            "Test".to_string(),
        )
        .unwrap();

        repo.guardar(&reserva).await.unwrap();

        assert!(repo
            .existe_para_empleado_en_slot("emp-001", &slot)
            .await
            .unwrap());
        assert!(!repo
            .existe_para_empleado_en_slot("emp-002", &slot)
            .await
            .unwrap());
    }
}
