use chrono::{DateTime, Datelike, Duration, Timelike, Utc};

/// Value Object: Slot de tiempo
/// Representa un bloque de tiempo de 1 hora
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Slot {
    pub inicio: DateTime<Utc>,
}

impl Slot {
    /// Crea un slot asegurándose que está al inicio de la hora
    pub fn new(fecha_hora: DateTime<Utc>) -> Self {
        // Redondear al inicio de la hora
        let inicio = fecha_hora
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();

        Self { inicio }
    }

    /// Crea un slot para una fecha y hora específicas
    pub fn from_date_and_hour(year: i32, month: u32, day: u32, hour: u32) -> Option<Self> {
        let fecha = chrono::NaiveDate::from_ymd_opt(year, month, day)?;
        let hora = chrono::NaiveTime::from_hms_opt(hour, 0, 0)?;
        let fecha_hora = chrono::DateTime::<Utc>::from_naive_utc_and_offset(
            chrono::NaiveDateTime::new(fecha, hora),
            Utc,
        );
        Some(Self::new(fecha_hora))
    }

    /// Obtiene el fin del slot (1 hora después)
    pub fn fin(&self) -> DateTime<Utc> {
        self.inicio + Duration::hours(1)
    }

    /// Verifica si este slot es en horario laboral (9:00 - 18:00)
    pub fn es_horario_laboral(&self) -> bool {
        let hora = self.inicio.hour();
        (9..18).contains(&hora)
    }

    /// Siguiente slot (1 hora después)
    pub fn siguiente(&self) -> Self {
        Self {
            inicio: self.inicio + Duration::hours(1),
        }
    }

    /// Obtiene todos los slots de un día laboral (9:00 - 18:00)
    pub fn slots_del_dia(fecha: DateTime<Utc>) -> Vec<Self> {
        let mut slots = Vec::new();
        for hora in 9..18 {
            if let Some(slot) =
                Self::from_date_and_hour(fecha.year(), fecha.month(), fecha.day(), hora)
            {
                slots.push(slot);
            }
        }
        slots
    }

    /// Formatea el slot para mostrar
    pub fn formato_legible(&self) -> String {
        format!(
            "{:04}-{:02}-{:02} {:02}:00-{:02}:00",
            self.inicio.year(),
            self.inicio.month(),
            self.inicio.day(),
            self.inicio.hour(),
            self.fin().hour()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_slot() {
        let slot = Slot::from_date_and_hour(2025, 11, 22, 10).unwrap();
        assert_eq!(slot.inicio.hour(), 10);
        assert_eq!(slot.inicio.minute(), 0);
    }

    #[test]
    fn test_slot_horario_laboral() {
        let slot_laboral = Slot::from_date_and_hour(2025, 11, 22, 14).unwrap();
        assert!(slot_laboral.es_horario_laboral());

        let slot_fuera = Slot::from_date_and_hour(2025, 11, 22, 20).unwrap();
        assert!(!slot_fuera.es_horario_laboral());
    }

    #[test]
    fn test_siguiente_slot() {
        let slot = Slot::from_date_and_hour(2025, 11, 22, 10).unwrap();
        let siguiente = slot.siguiente();
        assert_eq!(siguiente.inicio.hour(), 11);
    }

    #[test]
    fn test_slots_del_dia() {
        let fecha = Utc::now();
        let slots = Slot::slots_del_dia(fecha);
        assert_eq!(slots.len(), 9); // 9:00 a 17:00 (9 slots)
    }

    #[test]
    fn test_formato_legible() {
        let slot = Slot::from_date_and_hour(2025, 11, 22, 14).unwrap();
        let formato = slot.formato_legible();
        assert!(formato.contains("14:00-15:00"));
    }
}
