// CLI del Sistema de Reservas
// Este es un ADAPTADOR DE ENTRADA que interactúa con la API REST
//
// Este crate puede usarse tanto como librería (para reutilizar ApiClient y commands)
// como binario ejecutable a través del crate cli-app

pub mod api_client;
pub mod cli_args;
pub mod commands;

// Re-exportar tipos principales para facilitar el uso
pub use api_client::ApiClient;
pub use cli_args::{Cli, Commands, EmpleadoCommands, ReservaCommands};
