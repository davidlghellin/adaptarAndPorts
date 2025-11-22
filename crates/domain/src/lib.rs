// 🎯 DOMINIO - Núcleo de la aplicación
//
// Esta capa contiene:
// - Entidades de negocio
// - Reglas de negocio
// - Validaciones
//
// PRINCIPIO: El dominio NO conoce nada de infraestructura
// No depende de bases de datos, APIs, frameworks, etc.

pub mod reserva;

pub use reserva::{Reserva, EstadoReserva, ReservaError};
