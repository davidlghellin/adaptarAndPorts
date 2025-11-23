// Binario ejecutable del CLI
// Este es solo un thin wrapper que usa la librería reservas-cli

use clap::Parser;
use colored::Colorize;
use reservas_cli::cli_args::SalaCommands;
use reservas_cli::{commands, ApiClient, Cli, Commands, EmpleadoCommands, ReservaCommands};

fn main() {
    let cli = Cli::parse();

    println!("{}", "Sistema de Reservas - CLI".cyan().bold());
    println!("{} {}\n", "API URL:".bold(), cli.url);

    let client = ApiClient::new(cli.url);

    match cli.command {
        Commands::Empleado(cmd) => match cmd {
            EmpleadoCommands::Crear { nombre, email } => {
                commands::crear_empleado(&client, nombre, email);
            }
            EmpleadoCommands::Listar => {
                commands::listar_empleados(&client);
            }
            EmpleadoCommands::Obtener { id } => {
                commands::obtener_empleado(&client, id);
            }
            EmpleadoCommands::Activar { id } => {
                commands::activar_empleado(&client, id);
            }
            EmpleadoCommands::Desactivar { id } => {
                commands::desactivar_empleado(&client, id);
            }
        },

        Commands::Reserva(cmd) => match cmd {
            ReservaCommands::Crear {
                empleado_id,
                fecha,
                hora,
                descripcion,
            } => {
                commands::crear_reserva(&client, empleado_id, fecha, hora, descripcion);
            }
            ReservaCommands::Listar => {
                commands::listar_reservas(&client);
            }
            ReservaCommands::ListarEmpleado { empleado_id } => {
                commands::listar_reservas_empleado(&client, empleado_id);
            }
            ReservaCommands::Confirmar { id } => {
                commands::confirmar_reserva(&client, id);
            }
            ReservaCommands::Cancelar { id } => {
                commands::cancelar_reserva(&client, id);
            }
        },

        Commands::Disponibilidad { fecha } => {
            commands::ver_disponibilidad(&client, fecha);
        }

        Commands::Sala(cmd) => match cmd {
            SalaCommands::Listar => {
                commands::listar_salas(&client);
            }

            SalaCommands::Crear { nombre, capacidad } => {
                commands::crear_sala(&client, nombre, capacidad);
            }
        },
    }
}
