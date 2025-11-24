use async_trait::async_trait;
use reservas_domain::Empleado;

/// Puerto de salida (OUTPUT PORT): Define cómo persistir empleados
#[async_trait]
pub trait EmpleadoRepository {
    async fn guardar(&self, empleado: &Empleado) -> Result<(), String>;

    async fn obtener(&self, id: &str) -> Result<Option<Empleado>, String>;

    async fn listar(&self) -> Result<Vec<Empleado>, String>;

    async fn actualizar(&self, empleado: &Empleado) -> Result<(), String>;

    async fn existe(&self, id: &str) -> Result<bool, String>;
}
