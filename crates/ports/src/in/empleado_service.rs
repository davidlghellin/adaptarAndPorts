use async_trait::async_trait;
use reservas_domain::Empleado;

/// Puerto de entrada para gestión de empleados
#[async_trait]
pub trait EmpleadoService: Send + Sync {
    async fn crear_empleado(&self, nombre: String, email: String) -> Result<Empleado, String>;

    async fn obtener_empleado(&self, id: &str) -> Result<Option<Empleado>, String>;

    async fn listar_empleados(&self) -> Result<Vec<Empleado>, String>;

    async fn desactivar_empleado(&self, id: &str) -> Result<Empleado, String>;

    async fn activar_empleado(&self, id: &str) -> Result<Empleado, String>;
}
