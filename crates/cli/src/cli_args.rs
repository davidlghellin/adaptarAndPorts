// Definición de argumentos y comandos del CLI usando clap

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "reservas")]
#[command(about = "CLI para el Sistema de Reservas de Empleados", long_about = None)]
#[command(version)]
pub struct Cli {
    #[arg(
        short,
        long,
        default_value = "http://localhost:3000/api",
        global = true
    )]
    pub url: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Gestión de empleados
    #[command(subcommand)]
    Empleado(EmpleadoCommands),

    /// Gestión de reservas
    #[command(subcommand)]
    Reserva(ReservaCommands),

    /// Ver disponibilidad
    Disponibilidad {
        /// Fecha en formato YYYY-MM-DD
        #[arg(short, long)]
        fecha: String,
    },
}

#[derive(Subcommand)]
pub enum EmpleadoCommands {
    /// Crear un nuevo empleado
    Crear {
        /// Nombre del empleado
        #[arg(short, long)]
        nombre: String,

        /// Email del empleado
        #[arg(short, long)]
        email: String,
    },

    /// Listar todos los empleados
    Listar,

    /// Obtener información de un empleado
    Obtener {
        /// ID del empleado
        #[arg(short, long)]
        id: String,
    },

    /// Activar un empleado
    Activar {
        /// ID del empleado
        #[arg(short, long)]
        id: String,
    },

    /// Desactivar un empleado
    Desactivar {
        /// ID del empleado
        #[arg(short, long)]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum ReservaCommands {
    /// Crear una nueva reserva
    Crear {
        /// ID del empleado
        #[arg(short, long)]
        empleado_id: String,

        /// Fecha en formato YYYY-MM-DD
        #[arg(short, long)]
        fecha: String,

        /// Hora (9-17)
        #[arg(long)]
        hora: u32,

        /// Descripción de la reserva
        #[arg(short, long)]
        descripcion: String,
    },

    /// Listar todas las reservas
    Listar,

    /// Listar reservas de un empleado
    ListarEmpleado {
        /// ID del empleado
        #[arg(short, long)]
        empleado_id: String,
    },

    /// Confirmar una reserva
    Confirmar {
        /// ID de la reserva
        #[arg(short, long)]
        id: String,
    },

    /// Cancelar una reserva
    Cancelar {
        /// ID de la reserva
        #[arg(short, long)]
        id: String,
    },
}
