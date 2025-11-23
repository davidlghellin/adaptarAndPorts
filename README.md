# Sistema de Reservas - Arquitectura Hexagonal en Rust

Sistema de gestión de reservas implementado con arquitectura hexagonal (puertos y adaptadores) usando múltiples crates de Rust.

## 🚀 Quick Start

```bash
# Ejecutar el servidor
cargo run -p api-server

# Abrir en el navegador
open http://localhost:3000/          # Interfaz Web
open http://localhost:3000/api/swagger-ui  # API REST docs

# O usar la CLI
cargo run -p cli-app -- empleado crear --nombre "Ana" --email "ana@empresa.com"
cargo run -p cli-app -- empleado listar
```

## 🎯 Objetivo Educativo

Este proyecto está diseñado para aprender arquitectura hexagonal paso a paso, con separación clara de responsabilidades mediante crates independientes.

**Tres formas de interactuar con el mismo dominio**:
1. **Web UI** - Interfaz HTML simple en la raíz (`/`)
2. **API REST** - Endpoints JSON documentados bajo `/api`
3. **CLI** - Cliente de línea de comandos

Todos comparten los mismos servicios y repositorios, demostrando la flexibilidad de la arquitectura hexagonal.

## 📦 Estructura de Crates

### [crates/domain](crates/domain/) - El Núcleo
**Sin dependencias de infraestructura**
- Entidades de negocio (`Empleado`, `Reserva`, `Slot`)
- Reglas de negocio (validaciones, estados)
- Estados (`Pendiente`, `Confirmada`, `Cancelada`)

### [crates/ports](crates/ports/) - Los Contratos
**Depende solo de: `domain`**
- `EmpleadoService` y `ReservaService` (puertos de entrada - casos de uso)
- `EmpleadoRepository` y `ReservaRepository` (puertos de salida - persistencia)

### [crates/application](crates/application/) - Casos de Uso
**Depende de: `domain`, `ports`**
- `EmpleadoServiceImpl` y `ReservaServiceImpl` - Implementan los casos de uso
- Orquesta el dominio con los repositorios

### [crates/adapters](crates/adapters/) - Implementaciones
**Depende de: `domain`, `ports`**
- `InMemoryEmpleadoRepository` y `InMemoryReservaRepository` - Repositorios en memoria con HashMap
- Futuros: PostgreSQL, MongoDB, etc.

### [crates/api-rest](crates/api-rest/) - API REST
**Depende de: `domain`, `ports`**
- API REST con Axum
- Documentación OpenAPI/Swagger
- Endpoints JSON para empleados y reservas

### [crates/web-ui](crates/web-ui/) - Interfaz Web
**Depende de: `domain`, `ports`**
- Interfaz HTML simple con Askama templates
- Sin JavaScript, server-side rendering
- Páginas para gestionar empleados y reservas

### [crates/cli](crates/cli/) - CLI Interactiva
**Depende de: `domain`, `ports`**
- Cliente CLI que consume la API REST
- Comandos para gestionar empleados y reservas

### [api-server](api-server/) - Servidor HTTP
**Depende de: `application`, `adapters`, `api-rest`, `web-ui`**
- Ensambla la API REST y la Web UI
- Inyección de dependencias
- Servidor HTTP unificado

### [cli-app](cli-app/) - Aplicación CLI
**Depende de: `cli`**
- Ejecutable de línea de comandos
- Cliente para consumir la API

## 🚀 Comandos

### Ejecutar el servidor HTTP (API REST + Web UI):
```bash
cargo run -p api-server
```

Esto inicia el servidor en `http://localhost:3000` con:
- **Interfaz Web**: http://localhost:3000/
- **API REST**: http://localhost:3000/api/...
- **Swagger UI**: http://localhost:3000/api/swagger-ui

### Usar la CLI:
```bash
# Listar empleados
cargo run -p cli-app -- empleado listar

# Crear empleado
cargo run -p cli-app -- empleado crear --nombre "Juan López" --email "juan@empresa.com"

# Activar/desactivar empleado
cargo run -p cli-app -- empleado activar --id <empleado-id>
cargo run -p cli-app -- empleado desactivar --id <empleado-id>

# Listar reservas
cargo run -p cli-app -- reserva listar

# Crear reserva
cargo run -p cli-app -- reserva crear --empleado-id <id> --fecha 2025-11-25 --hora 9 --descripcion "Reunión"

# Confirmar/cancelar reserva
cargo run -p cli-app -- reserva confirmar --id <reserva-id>
cargo run -p cli-app -- reserva cancelar --id <reserva-id>

# Ver disponibilidad
cargo run -p cli-app -- disponibilidad --fecha 2025-11-25
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
cargo tree -p api-server
```

### Compilar todo:
```bash
cargo build
```

## 📚 Documentación

- [ARQUITECTURA.md](ARQUITECTURA.md) - Conceptos y diagramas de arquitectura hexagonal
- [CRATES.md](CRATES.md) - Explicación de la estructura multi-crate

## 🌐 Interfaz Web

La interfaz web está disponible en la raíz del servidor (`http://localhost:3000/`):

- **Página principal**: Dashboard con acceso a todas las secciones
- **Gestión de Empleados**: Crear, listar, activar/desactivar empleados
- **Gestión de Reservas**: Listar, confirmar y cancelar reservas
- **Diseño simple**: HTML básico con CSS, sin JavaScript

Características:
- ✅ Server-side rendering con Askama templates
- ✅ Formularios HTML nativos
- ✅ Integración completa con los servicios de aplicación
- ✅ Todo en Rust, sin dependencias de frontend

## 🔌 API REST

La API REST está disponible bajo `/api` con documentación interactiva:

**Empleados**:
- `POST /api/empleados` - Crear empleado
- `GET /api/empleados` - Listar empleados
- `GET /api/empleados/:id` - Obtener empleado
- `POST /api/empleados/:id/activar` - Activar empleado
- `POST /api/empleados/:id/desactivar` - Desactivar empleado

**Reservas**:
- `POST /api/reservas` - Crear reserva
- `GET /api/reservas` - Listar reservas
- `GET /api/reservas/:id` - Obtener reserva
- `POST /api/reservas/:id/confirmar` - Confirmar reserva
- `POST /api/reservas/:id/cancelar` - Cancelar reserva
- `GET /api/empleados/:id/reservas` - Reservas de un empleado

**Disponibilidad**:
- `GET /api/disponibilidad?fecha=YYYY-MM-DD` - Tabla de disponibilidad

## ✅ Tests Incluidos

**Dominio**:
- Creación y validación de empleados
- Creación y validación de reservas
- Validación de slots horarios
- Confirmación y cancelación de reservas

**Adaptadores**:
- Guardar y obtener empleados
- Guardar y obtener reservas
- Actualizar estados

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
// Creamos los adaptadores concretos
let empleado_repository = InMemoryEmpleadoRepository::new();
let reserva_repository = InMemoryReservaRepository::new();

// Los inyectamos en los servicios
let empleado_service = EmpleadoServiceImpl::new(empleado_repository);
let reserva_service = ReservaServiceImpl::new(reserva_repository);

// Usamos los servicios a través de las interfaces
empleado_service.crear_empleado(...).await?;
reserva_service.crear_reserva(...).await?;
```

### Ventajas

1. **El compilador fuerza la arquitectura** - Imposible violar las dependencias
2. **Testing independiente** - Cada crate se prueba por separado
3. **Reutilización** - Otros proyectos pueden usar solo el dominio
4. **Compilación paralela** - Rust compila crates independientes en paralelo
5. **Cambios localizados** - Cambiar de InMemory a Postgres no toca el dominio
6. **Múltiples adaptadores** - API REST, Web UI y CLI comparten los mismos servicios

## 🔄 Flujo de una Operación

```
1. Usuario → Web UI (/) o API REST (/api) o CLI
              ↓
2. Handler (web-ui/api-rest/cli)
   - Recibe petición HTTP o comando
              ↓
3. Service (application)
   - EmpleadoServiceImpl o ReservaServiceImpl
   - Genera UUID, valida datos
              ↓
4. Domain (domain)
   - Empleado::new() o Reserva::new()
   - Valida reglas de negocio
              ↓
5. Repository (ports → adapters)
   - repository.guardar(&entidad)
   - InMemoryRepository guarda en HashMap
              ↓
6. Respuesta → Usuario
   - JSON (API REST) o HTML (Web UI) o texto (CLI)
```

## ✨ Características Implementadas

- ✅ API REST con Axum y documentación Swagger/OpenAPI
- ✅ Interfaz Web HTML con Askama templates
- ✅ CLI interactiva que consume la API REST
- ✅ Gestión completa de empleados (crear, listar, activar/desactivar)
- ✅ Gestión completa de reservas (crear, listar, confirmar, cancelar)
- ✅ Tabla de disponibilidad por fecha
- ✅ Validación de slots horarios

## 🚧 Próximos Pasos

- [ ] Añadir adaptador PostgreSQL
- [ ] Validación de solapamiento de horarios
- [ ] Eventos de dominio
- [ ] Capacidad máxima del sistema
- [ ] Notificaciones por email
- [ ] Sistema de autenticación y autorización

## 📖 Para Aprender Más

- **Arquitectura Hexagonal**: Alistair Cockburn
- **Domain-Driven Design**: Eric Evans
- **Clean Architecture**: Robert C. Martin

## 🛠️ Tecnologías

**Core**:
- **Rust** 2021 edition
- **Tokio** - Runtime asíncrono
- **Chrono** - Manejo de fechas y horarios
- **UUID** - Generación de IDs únicos
- **Async-trait** - Traits asíncronos

**API REST**:
- **Axum** 0.7 - Framework web
- **Utoipa** - Generación de OpenAPI/Swagger
- **Serde** - Serialización JSON

**Web UI**:
- **Askama** - Templates HTML (similar a Jinja)
- **Tower-HTTP** - Servir archivos estáticos
- **CSS** vanilla - Sin frameworks frontend

**CLI**:
- **Clap** - Parser de argumentos
- **Reqwest** - Cliente HTTP para consumir la API
