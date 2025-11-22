// 🎯 DOMINIO - Núcleo de la aplicación
//
// Esta capa contiene:
// - Entidades de negocio (Reserva, Empleado)
// - Value Objects (Slot)
// - Servicios de dominio
// - Reglas de negocio
// - Validaciones
//
// PRINCIPIO: El dominio NO conoce nada de infraestructura
// No depende de bases de datos, APIs, frameworks, etc.

pub mod disponibilidad;
pub mod empleado;
pub mod reserva;
pub mod slot;

pub use disponibilidad::DisponibilidadService;
pub use empleado::Empleado;
pub use reserva::{EstadoReserva, Reserva, ReservaError};
pub use slot::Slot;
