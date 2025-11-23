# Guía de Desarrollo - Sistema de Reservas

Esta guía documenta cómo se construyó el proyecto y cómo agregar nuevas funcionalidades siguiendo la arquitectura hexagonal.

## 📋 Tabla de Contenidos

- [Cómo se construyó el proyecto](#cómo-se-construyó-el-proyecto)
- [Estructura de la Arquitectura Hexagonal](#estructura-de-la-arquitectura-hexagonal)
- [Cómo agregar una nueva entidad](#cómo-agregar-una-nueva-entidad)
- [Cómo agregar una nueva pantalla Web](#cómo-agregar-una-nueva-pantalla-web)
- [Cómo agregar un nuevo endpoint API](#cómo-agregar-un-nuevo-endpoint-api)
- [Cómo agregar un comando CLI](#cómo-agregar-un-comando-cli)
- [Flujo de desarrollo completo](#flujo-de-desarrollo-completo)

---

## 🏗️ Cómo se construyó el proyecto

### Paso 1: Crear la estructura de crates

```bash
# Workspace raíz
cargo init --lib

# Crates del núcleo hexagonal (de dentro hacia afuera)
cargo new --lib crates/domain      # Entidades y reglas de negocio
cargo new --lib crates/ports       # Contratos (traits)
cargo new --lib crates/application # Implementación de casos de uso
cargo new --lib crates/adapters    # Repositorios en memoria

# Adaptadores de entrada
cargo new --lib crates/api-rest    # API REST con Axum
cargo new --lib crates/web-ui      # Interfaz Web con Askama
cargo new --lib crates/cli         # Cliente CLI

# Aplicaciones ejecutables
cargo new api-server               # Servidor HTTP
cargo new cli-app                  # Aplicación CLI
```

### Paso 2: Configurar el workspace

Editar `Cargo.toml` en la raíz:

```toml
[workspace]
members = [
    "crates/domain",
    "crates/ports",
    "crates/application",
    "crates/adapters",
    "crates/api-rest",
    "crates/web-ui",
    "crates/cli",
    "api-server",
    "cli-app",
]
resolver = "2"

[workspace.dependencies]
tokio = { version = "1.0", features = ["full"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
```

### Paso 3: Implementar el núcleo (Domain → Ports → Application)

**Orden de implementación:**

1. **Domain** (crates/domain) - Sin dependencias externas

   - Crear entidades (`Empleado`, `Reserva`, `Slot`)
   - Implementar validaciones
   - Definir enums (`EstadoReserva`)

2. **Ports** (crates/ports) - Depende solo de domain

   - Definir traits de servicios (`EmpleadoService`, `ReservaService`)
   - Definir traits de repositorios (`EmpleadoRepository`, `ReservaRepository`)

3. **Application** (crates/application) - Depende de domain y ports

   - Implementar servicios (`EmpleadoServiceImpl`, `ReservaServiceImpl`)
   - Orquestar lógica de negocio

4. **Adapters** (crates/adapters) - Depende de domain y ports
   - Implementar repositorios (`InMemoryEmpleadoRepository`, `InMemoryReservaRepository`)

### Paso 4: Implementar adaptadores de entrada

**API REST** (crates/api-rest):

```bash
cd crates/api-rest
# Agregar dependencias en Cargo.toml
```

**Web UI** (crates/web-ui):

```bash
cd crates/web-ui
# Crear carpetas
mkdir -p templates static src
```

**CLI** (crates/cli):

```bash
cd crates/cli
# Implementar cliente HTTP y comandos
```

### Paso 5: Ensamblar las aplicaciones

**API Server** (api-server/src/main.rs):

```rust
// 1. Crear repositorios
let empleado_repo = InMemoryEmpleadoRepository::new();
let reserva_repo = InMemoryReservaRepository::new();

// 2. Crear servicios
let empleado_service = EmpleadoServiceImpl::new(empleado_repo);
let reserva_service = ReservaServiceImpl::new(reserva_repo);

// 3. Crear routers
let api_router = api_rest::crear_router(...);
let web_router = web_ui::crear_router_web(...);

// 4. Combinar y servir
let app = web_router.merge(Router::new().nest("/api", api_router));
```

---

## 🎯 Estructura de la Arquitectura Hexagonal

```
┌─────────────────────────────────────────────────────────────┐
│                    ADAPTADORES DE ENTRADA                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Web UI     │  │   API REST   │  │     CLI      │      │
│  │  (Askama)    │  │   (Axum)     │  │   (Clap)     │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
└─────────┼──────────────────┼──────────────────┼─────────────┘
          │                  │                  │
          └──────────────────┼──────────────────┘
                             ▼
          ┌─────────────────────────────────────┐
          │         PUERTOS DE ENTRADA          │
          │  (EmpleadoService, ReservaService)  │
          └─────────────────┬───────────────────┘
                            ▼
          ┌─────────────────────────────────────┐
          │           APLICACIÓN                │
          │  (EmpleadoServiceImpl, etc.)        │
          └─────────────────┬───────────────────┘
                            ▼
          ┌─────────────────────────────────────┐
          │            DOMINIO                  │
          │  (Empleado, Reserva, Slot)          │
          │  (Reglas de negocio)                │
          └─────────────────┬───────────────────┘
                            ▼
          ┌─────────────────────────────────────┐
          │        PUERTOS DE SALIDA            │
          │  (EmpleadoRepo, ReservaRepo)        │
          └─────────────────┬───────────────────┘
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   ADAPTADORES DE SALIDA                      │
│           (InMemoryRepo, PostgresRepo, etc.)                 │
└─────────────────────────────────────────────────────────────┘
```

**Reglas de dependencias:**

- Domain: **NO** depende de nada
- Ports: Depende solo de Domain
- Application: Depende de Domain y Ports
- Adapters: Depende de Domain y Ports (pero NO de Application)
- Aplicaciones (api-server, cli-app): Ensamblan todo

---

## ➕ Cómo agregar una nueva entidad

Ejemplo: Agregar una entidad `Sala` (sala de reuniones)

### 1. Domain (crates/domain/src/sala.rs)

```rust
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Sala {
    pub id: String,
    pub nombre: String,
    pub capacidad: u32,
    pub activa: bool,
}

impl Sala {
    pub fn new(id: String, nombre: String, capacidad: u32) -> Result<Self, String> {
        if nombre.trim().is_empty() {
            return Err("El nombre no puede estar vacío".to_string());
        }
        if capacidad == 0 {
            return Err("La capacidad debe ser mayor a 0".to_string());
        }

        Ok(Self {
            id,
            nombre,
            capacidad,
            activa: true,
        })
    }

    pub fn desactivar(&mut self) {
        self.activa = false;
    }

    pub fn activar(&mut self) {
        self.activa = true;
    }
}
```

Exportar en `crates/domain/src/lib.rs`:

```rust
pub mod sala;
pub use sala::Sala;
```

### 2. Ports (crates/ports/src/sala.rs)

**Repository trait:**

```rust
use async_trait::async_trait;
use reservas_domain::Sala;

#[async_trait]
pub trait SalaRepository {
    async fn guardar(&self, sala: &Sala) -> Result<(), String>;
    async fn obtener(&self, id: &str) -> Result<Option<Sala>, String>;
    async fn listar(&self) -> Result<Vec<Sala>, String>;
    async fn actualizar(&self, sala: &Sala) -> Result<(), String>;
}
```

**Service trait:**

```rust
use async_trait::async_trait;
use reservas_domain::Sala;

#[async_trait]
pub trait SalaService: Send + Sync {
    async fn crear_sala(&self, nombre: String, capacidad: u32) -> Result<Sala, String>;
    async fn listar_salas(&self) -> Result<Vec<Sala>, String>;
    async fn obtener_sala(&self, id: &str) -> Result<Option<Sala>, String>;
    async fn activar_sala(&self, id: &str) -> Result<(), String>;
    async fn desactivar_sala(&self, id: &str) -> Result<(), String>;
}
```

Exportar en `crates/ports/src/lib.rs`:

```rust
pub mod sala;
pub use sala::{SalaRepository, SalaService};
```

### 3. Application (crates/application/src/sala_service_impl.rs)

```rust
use async_trait::async_trait;
use reservas_domain::Sala;
use reservas_ports::{SalaRepository, SalaService};
use std::sync::Arc;
use uuid::Uuid;

pub struct SalaServiceImpl<R: SalaRepository> {
    repository: R,
}

impl<R: SalaRepository> SalaServiceImpl<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: SalaRepository + Send + Sync> SalaService for SalaServiceImpl<R> {
    async fn crear_sala(&self, nombre: String, capacidad: u32) -> Result<Sala, String> {
        let id = Uuid::new_v4().to_string();
        let sala = Sala::new(id, nombre, capacidad)?;
        self.repository.guardar(&sala).await?;
        Ok(sala)
    }

    async fn listar_salas(&self) -> Result<Vec<Sala>, String> {
        self.repository.listar().await
    }

    async fn obtener_sala(&self, id: &str) -> Result<Option<Sala>, String> {
        self.repository.obtener(id).await
    }

    async fn activar_sala(&self, id: &str) -> Result<(), String> {
        let mut sala = self
            .repository
            .obtener(id)
            .await?
            .ok_or("Sala no encontrada")?;
        sala.activar();
        self.repository.actualizar(&sala).await
    }

    async fn desactivar_sala(&self, id: &str) -> Result<(), String> {
        let mut sala = self
            .repository
            .obtener(id)
            .await?
            .ok_or("Sala no encontrada")?;
        sala.desactivar();
        self.repository.actualizar(&sala).await
    }
}
```

### 4. Adapters (crates/adapters/src/sala_repository_memory.rs)

```rust
use async_trait::async_trait;
use reservas_domain::Sala;
use reservas_ports::SalaRepository;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct InMemorySalaRepository {
    salas: Arc<RwLock<HashMap<String, Sala>>>,
}

impl Default for InMemorySalaRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemorySalaRepository {
    pub fn new() -> Self {
        Self {
            salas: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl SalaRepository for InMemorySalaRepository {
    async fn guardar(&self, sala: &Sala) -> Result<(), String> {
        let mut salas = self.salas.write().await;
        salas.insert(sala.id.clone(), sala.clone());
        Ok(())
    }

    async fn obtener(&self, id: &str) -> Result<Option<Sala>, String> {
        let salas = self.salas.read().await;
        Ok(salas.get(id).cloned())
    }

    async fn listar(&self) -> Result<Vec<Sala>, String> {
        let salas = self.salas.read().await;
        Ok(salas.values().cloned().collect())
    }

    async fn actualizar(&self, sala: &Sala) -> Result<(), String> {
        let mut salas = self.salas.write().await;
        if salas.contains_key(&sala.id) {
            salas.insert(sala.id.clone(), sala.clone());
            Ok(())
        } else {
            Err("Sala no encontrada".to_string())
        }
    }
}
```

---

## 🌐 Cómo agregar una nueva pantalla Web

Ejemplo: Agregar página de listado de salas

### 1. Crear el template (crates/web-ui/templates/salas.html)

```html
{% extends "base.html" %}

{% block title %}Salas - Sistema de Reservas{% endblock %}

{% block content %}
<h2>Lista de Salas</h2>

<div class="actions">
    <a href="/salas/nuevo" class="btn btn-primary">Crear Nueva Sala</a>
</div>

{% if salas.is_empty() %}
<div class="empty-state">
    <p>No hay salas registradas todavía.</p>
    <a href="/salas/nuevo" class="btn btn-primary">Crear la primera</a>
</div>
{% else %}
<table class="data-table">
    <thead>
        <tr>
            <th>Nombre</th>
            <th>Capacidad</th>
            <th>Estado</th>
            <th>Acciones</th>
        </tr>
    </thead>
    <tbody>
        {% for sala in salas %}
        <tr>
            <td>{{ sala.nombre }}</td>
            <td>{{ sala.capacidad }} personas</td>
            <td>
                {% if sala.activa %}
                <span class="badge badge-success">Activa</span>
                {% else %}
                <span class="badge badge-danger">Inactiva</span>
                {% endif %}
            </td>
            <td>
                {% if sala.activa %}
                <form method="post" action="/salas/{{ sala.id }}/desactivar" style="display: inline;">
                    <button type="submit" class="btn btn-sm btn-danger">Desactivar</button>
                </form>
                {% else %}
                <form method="post" action="/salas/{{ sala.id }}/activar" style="display: inline;">
                    <button type="submit" class="btn btn-sm btn-success">Activar</button>
                </form>
                {% endif %}
            </td>
        </tr>
        {% endfor %}
    </tbody>
</table>
{% endif %}
{% endblock %}
```

### 2. Definir structs para templates (crates/web-ui/src/templates.rs)

```rust
use askama::Template;

#[derive(Debug)]
pub struct SalaView {
    pub id: String,
    pub nombre: String,
    pub capacidad: u32,
    pub activa: bool,
}

#[derive(Template)]
#[template(path = "salas.html")]
pub struct SalasTemplate {
    pub salas: Vec<SalaView>,
}
```

### 3. Crear handlers (crates/web-ui/src/handlers.rs)

```rust
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use reservas_ports::SalaService;
use std::sync::Arc;
use crate::templates::{SalasTemplate, SalaView};


pub async fn listar_salas_page(
    Extension(service): Extension<Arc<dyn SalaService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let salas = service
        .listar_salas()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let salas_view: Vec<SalaView> = salas
        .into_iter()
        .map(|s| SalaView {
            id: s.id,
            nombre: s.nombre,
            capacidad: s.capacidad,
            activa: s.activa,
        })
        .collect();

    Ok(SalasTemplate { salas: salas_view })
}

pub async fn activar_sala(
    Extension(service): Extension<Arc<dyn SalaService>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    service
        .activar_sala(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/salas"))
}

pub async fn desactivar_sala(
    Extension(service): Extension<Arc<dyn SalaService>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    service
        .desactivar_sala(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/salas"))
}
```

### 4. Agregar rutas (crates/web-ui/src/routes.rs)

```rust
use axum::{
    routing::{get, post},
    Router,
};

pub fn crear_router_web(
    sala_service: Arc<dyn SalaService>,
    // ... otros servicios
) -> Router {
    Router::new()
        // ... rutas existentes
        .route("/salas", get(handlers::listar_salas_page))
        .route("/salas/:id/activar", post(handlers::activar_sala))
        .route("/salas/:id/desactivar", post(handlers::desactivar_sala))
        .layer(axum::Extension(sala_service))
        // ... otros layers
}
```

### 5. Actualizar navegación (crates/web-ui/templates/base.html)

```html
<nav class="navbar">
  <div class="container">
    <h1>Sistema de Reservas</h1>
    <ul>
      <li><a href="/">Inicio</a></li>
      <li><a href="/empleados">Empleados</a></li>
      <li><a href="/salas">Salas</a></li><!-- NUEVA -->
      <li><a href="/reservas">Reservas</a></li>
      <li><a href="/disponibilidad">Disponibilidad</a></li>
    </ul>
  </div>
</nav>
```

### 6. Integrar en api-server (api-server/src/main.rs)

```rust
// Crear repositorio y servicio
let sala_repository: InMemorySalaRepository = InMemorySalaRepository::new();
let sala_service: Arc<dyn SalaService> =
        Arc::new(SalaServiceImpl::new(sala_repository)) as Arc<dyn reservas_ports::SalaService>;

// añadimos al api_router
let api_router =
        api_rest::crear_router(
            Arc::clone(&empleado_service), 
            Arc::clone(&reserva_service),
            Arc::clone(&sala_service) // nueva
        );

// Pasar al router
let web_router = web_ui::crear_router_web(
    Arc::clone(&empleado_service),
    Arc::clone(&reserva_service),
    Arc::clone(&sala_service), // nueva
);
```

---

## 🔌 Cómo agregar un nuevo endpoint API

Ejemplo: Endpoint para obtener salas activas

### 1. Definir DTOs (crates/api-rest/src/dtos.rs)

```rust
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CrearSalaRequest {
    pub nombre: String,
    pub capacidad: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SalaResponse {
    pub id: String,
    pub nombre: String,
    pub capacidad: u32,
    pub activa: bool,
}
```

### 2. Crear handlers (crates/api-rest/src/handlers.rs)

```rust
use crate::dtos::{CrearSalaRequest, ErrorResponse, SalaResponse};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use reservas_ports::SalaService;
use std::sync::Arc;

/// Listar todas las salas
#[utoipa::path(
    get,
    path = "/salas",
    responses(
        (status = 200, description = "Lista de salas", body = [SalaResponse])
    ),
    tag = "Salas"
)]
pub async fn listar_salas(Extension(service): Extension<Arc<dyn SalaService>>) -> Response {
    match service.listar_salas().await {
        Ok(salas) => {
            let response: Vec<SalaResponse> = salas.into_iter().map(|e| e.into()).collect();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: e }),
        )
            .into_response(),
    }
}

/// Crear nueva sala
#[utoipa::path(
    post,
    path = "/salas",
    request_body = CrearSalaRequest,
    responses(
        (status = 201, description = "Sala creada", body = SalaResponse)
    ),
    tag = "Salas"
)]
pub async fn crear_sala(
    Extension(service): Extension<Arc<dyn SalaService>>,
    Json(request): Json<CrearSalaRequest>,
) -> Result<(StatusCode, Json<SalaResponse>), StatusCode> {
    let sala = service
        .crear_sala(request.nombre, request.capacidad)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let response = SalaResponse {
        id: sala.id,
        nombre: sala.nombre,
        capacidad: sala.capacidad,
        activa: sala.activa,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
```

### 3. Agregar rutas (crates/api-rest/src/lib.rs)

```rust
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // ... paths existentes
        handlers::listar_salas,
        handlers::crear_sala,
    ),
    components(
        schemas(
            // ... schemas existentes
            dtos::CrearSalaRequest,
            dtos::SalaResponse,
        )
    ),
    tags(
        (name = "Empleados", description = "Gestión de empleados"),
        (name = "Reservas", description = "Gestión de reservas"),
        (name = "Salas", description = "Gestión de salas"),  // NUEVA
    )
)]
struct ApiDoc;

pub fn crear_router(
    sala_service: Arc<dyn SalaService>,
    // ... otros servicios
) -> Router {
    let api_router = Router::new()
        // ... rutas existentes
        .route("/salas", get(handlers::listar_salas))
        .route("/salas", post(handlers::crear_sala))
        .layer(Extension(sala_service));

    // ... resto del código
}
```

---

## 🖥️ Cómo agregar un comando CLI

Ejemplo: Comando para listar salas

### 1. Definir argumentos (crates/cli/src/cli_args.rs)

```rust
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum SalaCommands {
    /// Listar todas las salas
    Listar,
    /// Crear nueva sala
    Crear {
        nombre: String,
        capacidad: u32,
    },
    /// Activar sala
    Activar { id: String },
    /// Desactivar sala
    Desactivar { id: String },
}

#[derive(Subcommand)]
pub enum Commands {
    Empleado {
        #[command(subcommand)]
        comando: EmpleadoCommands,
    },
    Reserva {
        #[command(subcommand)]
        comando: ReservaCommands,
    },
    Sala {  // NUEVO
        #[command(subcommand)]
        comando: SalaCommands,
    },
}
```

### 2. Implementar comandos (crates/cli/src/commands/sala.rs)

```rust
use crate::{api_client::ApiClient, cli_args::SalaCommands};

pub async fn ejecutar_comando_sala(
    client: &ApiClient,
    comando: SalaCommands,
) -> Result<(), Box<dyn std::error::Error>> {
    match comando {
        SalaCommands::Listar => {
            let salas = client.listar_salas().await?;

            if salas.is_empty() {
                println!("No hay salas registradas");
            } else {
                println!("\n{:<36} {:<30} {:<12} {:<10}",
                    "ID", "Nombre", "Capacidad", "Estado");
                println!("{}", "-".repeat(90));

                for sala in salas {
                    let estado = if sala.activa { "Activa" } else { "Inactiva" };
                    println!("{:<36} {:<30} {:<12} {:<10}",
                        sala.id, sala.nombre, sala.capacidad, estado);
                }
            }
        }
        SalaCommands::Crear { nombre, capacidad } => {
            let sala = client.crear_sala(nombre, capacidad).await?;
            println!("✓ Sala creada con ID: {}", sala.id);
        }
        SalaCommands::Activar { id } => {
            client.activar_sala(&id).await?;
            println!("✓ Sala activada");
        }
        SalaCommands::Desactivar { id } => {
            client.desactivar_sala(&id).await?;
            println!("✓ Sala desactivada");
        }
    }
    Ok(())
}
```

### 3. Agregar cliente API (crates/cli/src/api_client.rs)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SalaDto {
    pub id: String,
    pub nombre: String,
    pub capacidad: u32,
    pub activa: bool,
}

impl ApiClient {
    pub async fn listar_salas(&self) -> Result<Vec<SalaDto>, Box<dyn std::error::Error>> {
        let url = format!("{}/salas", self.base_url);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let salas = response.json().await?;
            Ok(salas)
        } else {
            Err(format!("Error: {}", response.status()).into())
        }
    }

    pub async fn crear_sala(
        &self,
        nombre: String,
        capacidad: u32,
    ) -> Result<SalaDto, Box<dyn std::error::Error>> {
        let url = format!("{}/salas", self.base_url);
        let body = serde_json::json!({
            "nombre": nombre,
            "capacidad": capacidad,
        });

        let response = self.client.post(&url).json(&body).send().await?;

        if response.status().is_success() {
            let sala = response.json().await?;
            Ok(sala)
        } else {
            Err(format!("Error: {}", response.status()).into())
        }
    }
}
```

### 4. Integrar en main (cli-app/src/main.rs)

```rust
use cli::{Commands, SalaCommands};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = ApiClient::new(&cli.api_url);

    match &cli.command {
        Commands::Empleado { comando } => {
            ejecutar_comando_empleado(&client, comando.clone()).await?;
        }
        Commands::Reserva { comando } => {
            ejecutar_comando_reserva(&client, comando.clone()).await?;
        }
        Commands::Sala { comando } => {  // NUEVO
            ejecutar_comando_sala(&client, comando.clone()).await?;
        }
    }

    Ok(())
}
```

---

## 🔄 Flujo de desarrollo completo

Cuando agregues una nueva funcionalidad completa (entidad + Web + API + CLI):

### Checklist de desarrollo

1. **Domain** ✓

   - [ ] Crear entidad con validaciones
   - [ ] Escribir tests unitarios
   - [ ] Exportar en lib.rs

2. **Ports** ✓

   - [ ] Definir trait Repository
   - [ ] Definir trait Service
   - [ ] Exportar en lib.rs

3. **Application** ✓

   - [ ] Implementar ServiceImpl
   - [ ] Escribir tests de integración
   - [ ] Exportar en lib.rs

4. **Adapters** ✓

   - [ ] Implementar InMemoryRepository
   - [ ] Escribir tests
   - [ ] Exportar en lib.rs

5. **API REST** ✓

   - [ ] Crear DTOs con ToSchema
   - [ ] Crear handlers con utoipa::path
   - [ ] Agregar rutas al router
   - [ ] Actualizar OpenAPI tags
   - [ ] Probar con curl o Swagger UI

6. **Web UI** ✓

   - [ ] Crear templates HTML
   - [ ] Crear structs de templates
   - [ ] Implementar handlers
   - [ ] Agregar rutas
   - [ ] Actualizar navegación
   - [ ] Probar en navegador

7. **CLI** ✓

   - [ ] Definir comandos en cli_args.rs
   - [ ] Implementar lógica de comandos
   - [ ] Agregar métodos al ApiClient
   - [ ] Integrar en main.rs
   - [ ] Probar comandos

8. **Integración** ✓
   - [ ] Agregar al api-server/main.rs
   - [ ] Actualizar README.md
   - [ ] Actualizar DEVELOPER.md
   - [ ] Ejecutar todos los tests: `cargo test`
   - [ ] Compilar todo: `cargo build`

### Orden recomendado

```
1. Domain (entidad + tests)
2. Ports (traits)
3. Application (implementación + tests)
4. Adapters (repository + tests)
5. API REST (endpoints + documentación)
6. Web UI (templates + handlers)
7. CLI (comandos)
8. Integración y documentación
```

### Comandos útiles durante desarrollo

```bash
# Compilar solo un crate
cargo build -p reservas-domain

# Ejecutar tests de un crate
cargo test -p reservas-application

# Ver dependencias
cargo tree -p api-server

# Ejecutar con logs
RUST_LOG=debug cargo run -p api-server

# Formatear código
cargo fmt

# Verificar código
cargo clippy

# Compilar en modo release
cargo build --release
```

---

## 🎨 Patrones y Convenciones

### Nombres de archivos

- `entidad.rs` - Entidades del dominio
- `entidad_service.rs` - Traits de servicios
- `entidad_service_impl.rs` - Implementación de servicios
- `entidad_repository.rs` - Traits de repositorios
- `entidad_repository_memory.rs` - Implementación en memoria

### Nombres de funciones

- **Services**: `crear_`, `listar_`, `obtener_`, `actualizar_`, `eliminar_`
- **Repositories**: `guardar`, `obtener`, `listar`, `actualizar`, `eliminar`
- **Handlers Web**: `_page` suffix para páginas, `_submit` para POST
- **Handlers API**: nombres descriptivos sin suffix

### Manejo de errores

- Domain: `Result<T, String>` con mensajes descriptivos
- Services: Propagar errores del domain y repository
- Handlers: Convertir a StatusCode apropiado
- CLI: `Result<(), Box<dyn std::error::Error>>`

### Tests

- Nombrar tests con `test_` prefix
- Un test por comportamiento
- Usar nombres descriptivos: `test_crear_empleado_valido`

---

## 📚 Recursos adicionales

- [Arquitectura Hexagonal](../ARQUITECTURA.md)
- [Estructura de Crates](../CRATES.md)
- [Documentación Axum](https://docs.rs/axum)
- [Documentación Askama](https://docs.rs/askama)
- [Documentación Utoipa](https://docs.rs/utoipa)

---

**Última actualización:** 2025-11-23
