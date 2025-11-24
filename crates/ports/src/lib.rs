// 🔌 PUERTOS - Interfaces que definen CONTRATOS
//
// Los puertos son como "enchufes" donde conectaremos los adaptadores
// Definen QUÉ se puede hacer, pero no CÓMO se hace
//
// Dos tipos:
// - INPUT PORTS: Cómo usar el sistema (casos de uso)
// - OUTPUT PORTS: Qué necesita el sistema (repositorios, etc.)

pub mod r#in;
pub mod out;
