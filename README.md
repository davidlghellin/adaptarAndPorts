# Sistema de Reservas - Arquitectura Hexagonal en Rust

Sistema de gestión de reservas implementado con arquitectura hexagonal (puertos y adaptadores) usando múltiples crates de Rust.

## 🎯 Objetivo Educativo

Este proyecto está diseñado para aprender arquitectura hexagonal paso a paso, con separación clara de responsabilidades mediante crates independientes.

## 📦 Estructura de Crates

### [crates/domain](crates/domain/) - El Núcleo
**Sin dependencias de infraestructura**
- Entidades de negocio (`Reserva`)
- Reglas de negocio (1-10 personas, fecha futura)
- Estados (`Pendiente`, `Confirmada`, `Cancelada`)

### [crates/ports](crates/ports/) - Los Contratos
**Depende solo de: `domain`**
- `ReservaService` (puerto de entrada - casos de uso)
- `ReservaRepository` (puerto de salida - persistencia)

### [crates/application](crates/application/) - Casos de Uso
**Depende de: `domain`, `ports`**
- `ReservaServiceImpl` - Implementa los casos de uso
- Orquesta el dominio con los repositorios

### [crates/adapters](crates/adapters/) - Implementaciones
**Depende de: `domain`, `ports`**
- `InMemoryReservaRepository` - Repositorio en memoria con HashMap
- Futuros: PostgreSQL, MongoDB, etc.

### [reservas-app](reservas-app/) - Binario Principal
**Depende de: todos**
- Ensambla la aplicación
- Inyección de dependencias
- Punto de entrada

## 🚀 Comandos

### Ejecutar la aplicación:
```bash
cargo run
```

### Ejecutar tests:
```bash
# Todos los tests:
cargo test

# Solo el dominio:
cargo test -p reservas-domain

# Solo los adaptadores:
cargo test -p reservas-adapters
```

### Ver el grafo de dependencias:
```bash
cargo tree -p reservas-app
```

### Compilar todo:
```bash
cargo build
```

## 📚 Documentación

- [ARQUITECTURA.md](ARQUITECTURA.md) - Conceptos y diagramas de arquitectura hexagonal
- [CRATES.md](CRATES.md) - Explicación de la estructura multi-crate

## ✅ Tests Incluidos

**Dominio** (3 tests):
- Creación de reserva válida
- Validación de número de personas
- Confirmación de reserva

**Adaptadores** (2 tests):
- Guardar y obtener reserva
- Actualizar estado de reserva

## 🎓 Conceptos Clave

### Principio de Inversión de Dependencias
El dominio NO conoce la infraestructura:
```rust
// ✅ Correcto: Application depende de Domain
use reservas_domain::Reserva;

// ❌ Imposible: Domain NO puede depender de Application
// El compilador lo evita!
```

### Inyección de Dependencias
```rust
// Creamos el adaptador concreto
let repository = InMemoryReservaRepository::new();

// Lo inyectamos en la aplicación
let service = ReservaServiceImpl::new(repository);

// Usamos el servicio a través de la interfaz
service.crear_reserva(...).await?;
```

### Ventajas

1. **El compilador fuerza la arquitectura** - Imposible violar las dependencias
2. **Testing independiente** - Cada crate se prueba por separado
3. **Reutilización** - Otros proyectos pueden usar solo el dominio
4. **Compilación paralela** - Rust compila crates independientes en paralelo
5. **Cambios localizados** - Cambiar de InMemory a Postgres no toca el dominio

## 🔄 Flujo de una Operación

```
1. Usuario → reservas-app/main.rs
              ↓
2. ReservaServiceImpl (application)
   - Genera UUID
   - Llama a Reserva::new() (domain)
              ↓
3. Reserva valida reglas de negocio (domain)
   - ¿1-10 personas? ✓
   - ¿Fecha futura? ✓
              ↓
4. Guarda usando el puerto (ports)
   repository.guardar(&reserva)
              ↓
5. InMemoryRepository (adapters)
   - Guarda en HashMap
```

## 🚧 Próximos Pasos

- [ ] Añadir adaptador PostgreSQL
- [ ] API REST con Axum
- [ ] CLI interactivo
- [ ] Validación de solapamiento de horarios
- [ ] Eventos de dominio
- [ ] Capacidad máxima del restaurante

## 📖 Para Aprender Más

- **Arquitectura Hexagonal**: Alistair Cockburn
- **Domain-Driven Design**: Eric Evans
- **Clean Architecture**: Robert C. Martin

## 🛠️ Tecnologías

- **Rust** 2021 edition
- **Tokio** - Runtime asíncrono
- **Chrono** - Manejo de fechas
- **UUID** - Generación de IDs únicos
- **Async-trait** - Traits asíncronos
