use chrono::{DateTime, Utc};

/// Entidad de dominio: Reserva
/// Esta es nuestra entidad central. NO depende de nada externo.
#[derive(Debug, Clone, PartialEq)]
pub struct Reserva {
    pub id: String,
    pub nombre_cliente: String,
    pub fecha: DateTime<Utc>,
    pub num_personas: u8,
    pub estado: EstadoReserva,
}

/// Estados posibles de una reserva
#[derive(Debug, Clone, PartialEq)]
pub enum EstadoReserva {
    Pendiente,
    Confirmada,
    Cancelada,
}

/// Errores del dominio
#[derive(Debug, PartialEq)]
pub enum ReservaError {
    NumeroPersonasInvalido,
    FechaInvalida,
    ReservaDuplicada,
}

impl Reserva {
    /// Constructor con validaciones de negocio
    pub fn new(
        id: String,
        nombre_cliente: String,
        fecha: DateTime<Utc>,
        num_personas: u8,
    ) -> Result<Self, ReservaError> {
        // Regla de negocio: mínimo 1 persona, máximo 10
        if num_personas == 0 || num_personas > 10 {
            return Err(ReservaError::NumeroPersonasInvalido);
        }

        // Regla de negocio: no se pueden hacer reservas en el pasado
        if fecha < Utc::now() {
            return Err(ReservaError::FechaInvalida);
        }

        Ok(Reserva {
            id,
            nombre_cliente,
            fecha,
            num_personas,
            estado: EstadoReserva::Pendiente,
        })
    }

    /// Lógica de negocio: confirmar reserva
    pub fn confirmar(&mut self) {
        self.estado = EstadoReserva::Confirmada;
    }

    /// Lógica de negocio: cancelar reserva
    pub fn cancelar(&mut self) {
        self.estado = EstadoReserva::Cancelada;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_reserva_valida() {
        let fecha = Utc::now() + chrono::Duration::days(1);
        let reserva = Reserva::new(
            "1".to_string(),
            "Juan Pérez".to_string(),
            fecha,
            4,
        );

        assert!(reserva.is_ok());
        let r = reserva.unwrap();
        assert_eq!(r.estado, EstadoReserva::Pendiente);
    }

    #[test]
    fn test_numero_personas_invalido() {
        let fecha = Utc::now() + chrono::Duration::days(1);

        let reserva_cero = Reserva::new("1".to_string(), "Test".to_string(), fecha, 0);
        assert_eq!(reserva_cero, Err(ReservaError::NumeroPersonasInvalido));

        let reserva_muchas = Reserva::new("1".to_string(), "Test".to_string(), fecha, 11);
        assert_eq!(reserva_muchas, Err(ReservaError::NumeroPersonasInvalido));
    }

    #[test]
    fn test_confirmar_reserva() {
        let fecha = Utc::now() + chrono::Duration::days(1);
        let mut reserva = Reserva::new("1".to_string(), "Test".to_string(), fecha, 2).unwrap();

        reserva.confirmar();
        assert_eq!(reserva.estado, EstadoReserva::Confirmada);
    }
}
