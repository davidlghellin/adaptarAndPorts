// Adaptador de salida: Repositorio de empleados en memoria

use async_trait::async_trait;
use reservas_domain::Empleado;
use reservas_ports::EmpleadoRepository;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct InMemoryEmpleadoRepository {
    storage: Arc<RwLock<HashMap<String, Empleado>>>,
}

impl InMemoryEmpleadoRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl EmpleadoRepository for InMemoryEmpleadoRepository {
    async fn guardar(&self, empleado: &Empleado) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        storage.insert(empleado.id.clone(), empleado.clone());
        Ok(())
    }

    async fn obtener(&self, id: &str) -> Result<Option<Empleado>, String> {
        let storage = self.storage.read().await;
        Ok(storage.get(id).cloned())
    }

    async fn listar(&self) -> Result<Vec<Empleado>, String> {
        let storage = self.storage.read().await;
        Ok(storage.values().cloned().collect())
    }

    async fn actualizar(&self, empleado: &Empleado) -> Result<(), String> {
        let mut storage = self.storage.write().await;

        if !storage.contains_key(&empleado.id) {
            return Err("Empleado no encontrado".to_string());
        }

        storage.insert(empleado.id.clone(), empleado.clone());
        Ok(())
    }

    async fn existe(&self, id: &str) -> Result<bool, String> {
        let storage = self.storage.read().await;
        Ok(storage.contains_key(id))
    }
}
