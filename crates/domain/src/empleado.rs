/// Entidad de dominio: Empleado
/// Representa un empleado que puede tener reservas asignadas
#[derive(Debug, Clone, PartialEq)]
pub struct Empleado {
    pub id: String,
    pub nombre: String,
    pub email: String,
    pub activo: bool,
}

impl Empleado {
    pub fn new(id: String, nombre: String, email: String) -> Self {
        Self {
            id,
            nombre,
            email,
            activo: true,
        }
    }

    pub fn desactivar(&mut self) {
        self.activo = false;
    }

    pub fn activar(&mut self) {
        self.activo = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_empleado() {
        let empleado = Empleado::new(
            "1".to_string(),
            "Juan López".to_string(),
            "juan@empresa.com".to_string(),
        );

        assert_eq!(empleado.nombre, "Juan López");
        assert!(empleado.activo);
    }

    #[test]
    fn test_desactivar_empleado() {
        let mut empleado = Empleado::new(
            "1".to_string(),
            "Juan".to_string(),
            "juan@empresa.com".to_string(),
        );

        empleado.desactivar();
        assert!(!empleado.activo);

        empleado.activar();
        assert!(empleado.activo);
    }
}
