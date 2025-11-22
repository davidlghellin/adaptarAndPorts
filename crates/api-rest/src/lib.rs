// API REST - Adaptador de entrada usando Axum
//
// Este crate es un ADAPTADOR DE ENTRADA que:
// 1. Expone endpoints HTTP
// 2. Convierte JSON a objetos de dominio
// 3. Llama a los puertos (servicios de aplicación)
// 4. Convierte respuestas de dominio a JSON
//
// IMPORTANTE: Este adaptador NO conoce los detalles de implementación
// Solo conoce los PUERTOS (traits) definidos en reservas-ports

pub mod dtos;
pub mod handlers;
pub mod mappers;
pub mod openapi;
pub mod routes;

pub use openapi::ApiDoc;
pub use routes::crear_router;
