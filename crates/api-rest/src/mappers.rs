// Mappers: Convierten entre DTOs (API) y entidades de dominio
// Esta es una capa de traducción importante en arquitectura hexagonal

use crate::dtos::*;
use chrono::Timelike;
use reservas_domain::{Empleado, EstadoReserva, Reserva, Sala, Slot};

// ============= Mappers de Empleado =============

impl From<Empleado> for EmpleadoResponse {
    fn from(empleado: Empleado) -> Self {
        EmpleadoResponse {
            id: empleado.id,
            nombre: empleado.nombre,
            email: empleado.email,
            activo: empleado.activo,
        }
    }
}

// ============= Mappers de Reserva =============

impl From<Reserva> for ReservaResponse {
    fn from(reserva: Reserva) -> Self {
        let estado_str = match reserva.estado {
            EstadoReserva::Pendiente => "pendiente",
            EstadoReserva::Confirmada => "confirmada",
            EstadoReserva::Cancelada => "cancelada",
        };

        ReservaResponse {
            id: reserva.id,
            empleado_id: reserva.empleado_id,
            slot_inicio: reserva.slot.inicio,
            slot_fin: reserva.slot.fin(),
            descripcion: reserva.descripcion,
            estado: estado_str.to_string(),
        }
    }
}

impl From<Sala> for SalaResponse {
    fn from(sala: Sala) -> Self {
        SalaResponse {
            id: sala.id,
            nombre: sala.nombre,
            capacidad: sala.capacidad,
            activa: true,
        }
    }
}
// ============= Mappers de Slot =============

impl From<&Slot> for SlotInfo {
    fn from(slot: &Slot) -> Self {
        SlotInfo {
            inicio: slot.inicio,
            fin: slot.fin(),
            hora: slot.inicio.hour(),
        }
    }
}
