// 🔌 PUERTOS - Interfaces que definen CONTRATOS
//
// Los puertos son como "enchufes" donde conectaremos los adaptadores
// Definen QUÉ se puede hacer, pero no CÓMO se hace
//
// Dos tipos:
// - INPUT PORTS: Cómo usar el sistema (casos de uso)
// - OUTPUT PORTS: Qué necesita el sistema (repositorios, etc.)

use reservas_domain::{Empleado, Reserva, Slot};
use async_trait::async_trait;

/// Puerto de entrada (INPUT PORT): Define cómo usar el sistema
/// Este es el "caso de uso" de nuestro sistema
#[async_trait]
pub trait ReservaService {
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

/// Puerto de entrada para gestión de empleados
#[async_trait]
pub trait EmpleadoService {
    async fn crear_empleado(&self, nombre: String, email: String) -> Result<Empleado, String>;

    async fn obtener_empleado(&self, id: &str) -> Result<Option<Empleado>, String>;

    async fn listar_empleados(&self) -> Result<Vec<Empleado>, String>;

    async fn desactivar_empleado(&self, id: &str) -> Result<Empleado, String>;
}

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

    async fn existe_para_empleado_en_slot(&self, empleado_id: &str, slot: &Slot) -> Result<bool, String>;
}

/// Puerto de salida (OUTPUT PORT): Define cómo persistir empleados
#[async_trait]
pub trait EmpleadoRepository {
    async fn guardar(&self, empleado: &Empleado) -> Result<(), String>;

    async fn obtener(&self, id: &str) -> Result<Option<Empleado>, String>;

    async fn listar(&self) -> Result<Vec<Empleado>, String>;

    async fn actualizar(&self, empleado: &Empleado) -> Result<(), String>;

    async fn existe(&self, id: &str) -> Result<bool, String>;
}
