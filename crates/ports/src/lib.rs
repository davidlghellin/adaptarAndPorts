// 🔌 PUERTOS - Interfaces que definen CONTRATOS
//
// Los puertos son como "enchufes" donde conectaremos los adaptadores
// Definen QUÉ se puede hacer, pero no CÓMO se hace
//
// Dos tipos:
// - INPUT PORTS: Cómo usar el sistema (casos de uso)
// - OUTPUT PORTS: Qué necesita el sistema (repositorios, etc.)

use reservas_domain::Reserva;
use async_trait::async_trait;

/// Puerto de entrada (INPUT PORT): Define cómo usar el sistema
/// Este es el "caso de uso" de nuestro sistema
#[async_trait]
pub trait ReservaService {
    async fn crear_reserva(
        &self,
        nombre_cliente: String,
        fecha: chrono::DateTime<chrono::Utc>,
        num_personas: u8,
    ) -> Result<Reserva, String>;

    async fn obtener_reserva(&self, id: &str) -> Result<Option<Reserva>, String>;

    async fn listar_reservas(&self) -> Result<Vec<Reserva>, String>;

    async fn confirmar_reserva(&self, id: &str) -> Result<Reserva, String>;

    async fn cancelar_reserva(&self, id: &str) -> Result<Reserva, String>;
}

/// Puerto de salida (OUTPUT PORT): Define cómo persistir datos
/// Este puerto será implementado por un adaptador (memoria, postgres, etc.)
#[async_trait]
pub trait ReservaRepository {
    async fn guardar(&self, reserva: &Reserva) -> Result<(), String>;

    async fn obtener(&self, id: &str) -> Result<Option<Reserva>, String>;

    async fn listar(&self) -> Result<Vec<Reserva>, String>;

    async fn actualizar(&self, reserva: &Reserva) -> Result<(), String>;

    async fn existe(&self, id: &str) -> Result<bool, String>;
}
