// 🔧 ADAPTADORES - Implementaciones CONCRETAS
//
// Los adaptadores conectan con tecnologías específicas:
// - Bases de datos (Postgres, MongoDB, etc.)
// - APIs externas
// - Sistemas de archivos
// - etc.
//
// Implementan los puertos (traits) usando tecnología real

pub mod empleado_repository_in_memory;
pub mod repository_in_memory;
pub mod sala_repository_memory;

pub use empleado_repository_in_memory::InMemoryEmpleadoRepository;
pub use repository_in_memory::InMemoryReservaRepository;
pub use sala_repository_memory::InMemorySalaRepository;
