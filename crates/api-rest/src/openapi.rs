// Configuración de OpenAPI/Swagger para la documentación de la API

use crate::dtos::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Sistema de Reservas - API REST",
        version = "1.0.0",
        description = "API REST para el sistema de reservas de empleados con arquitectura hexagonal (Puertos y Adaptadores)",
        contact(
            name = "Sistema de Reservas",
            email = "contacto@reservas.com"
        )
    ),
    servers(
        (url = "/api", description = "API REST Server")
    ),
    paths(
        crate::handlers::empleados::crear_empleado,
        crate::handlers::empleados::listar_empleados,
        crate::handlers::empleados::obtener_empleado,
        crate::handlers::empleados::desactivar_empleado,
        crate::handlers::empleados::activar_empleado,
        crate::handlers::reservas::crear_reserva,
        crate::handlers::reservas::listar_reservas,
        crate::handlers::reservas::obtener_reserva,
        crate::handlers::reservas::listar_reservas_empleado,
        crate::handlers::reservas::confirmar_reserva,
        crate::handlers::reservas::cancelar_reserva,
        crate::handlers::disponibilidad::obtener_disponibilidad,
    ),
    components(
        schemas(
            CrearEmpleadoRequest,
            EmpleadoResponse,
            CrearReservaRequest,
            ReservaResponse,
            DisponibilidadEmpleadoResponse,
            TablaDisponibilidadResponse,
            SlotInfo,
            ErrorResponse,
            MensajeResponse
        )
    ),
    tags(
        (name = "Empleados", description = "Gestión de empleados"),
        (name = "Reservas", description = "Gestión de reservas de tiempo"),
        (name = "Disponibilidad", description = "Consulta de disponibilidad de empleados")
    )
)]
pub struct ApiDoc;
