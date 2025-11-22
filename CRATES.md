# Arquitectura Multi-Crate

## Estructura del Workspace

```
adaptarAndPorts/
├── Cargo.toml                    (workspace root)
├── crates/
│   ├── domain/                   ⭐ Núcleo - SIN dependencias
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── reserva.rs
│   │
│   ├── ports/                    🔌 Contratos
│   │   ├── Cargo.toml            (depende: domain)
│   │   └── src/
│   │       └── lib.rs
│   │
│   ├── application/              ⚙️ Casos de uso
│   │   ├── Cargo.toml            (depende: domain, ports)
│   │   └── src/
│   │       └── lib.rs
│   │
│   └── adapters/                 🔧 Implementaciones
│       ├── Cargo.toml            (depende: domain, ports)
│       └── src/
│           ├── lib.rs
│           └── repository_in_memory.rs
│
└── reservas-app/                 🚀 Binario principal
    ├── Cargo.toml                (usa todos los crates)
    └── src/
        └── main.rs
```

## Grafo de Dependencias

```
                    ┌──────────────────┐
                    │  reservas-app    │
                    │   (binario)      │
                    └────────┬─────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
              ▼              ▼              ▼
    ┌────────────────┐  ┌────────────┐  ┌──────────────┐
    │   adapters     │  │application │  │    ports     │
    │                │  │            │  │              │
    └────┬───────┬───┘  └─────┬──────┘  └──────┬───────┘
         │       │            │                 │
         │       └────────────┼─────────────────┘
         │                    │
         │                    ▼
         │           ┌─────────────────┐
         └──────────▶│     domain      │  ⭐ NÚCLEO
                     │  (sin deps)     │
                     └─────────────────┘
```

## Ventajas de Esta Estructura

### 1. **Dependencias Forzadas por el Compilador**
```toml
# ❌ ESTO NO COMPILA:
# Si intentas en domain/Cargo.toml:
[dependencies]
reservas-ports = { path = "../ports" }  # ERROR: dependencia circular!
```

El compilador **impide** violar la arquitectura.

### 2. **Compilación Paralela**
Cargo compila los crates independientes en paralelo:
```
Compiling reservas-domain (sin dependencias) ✓
Compiling reservas-ports (espera domain) ✓
Compiling reservas-application + reservas-adapters (en paralelo) ✓
Compiling reservas-app (al final) ✓
```

### 3. **Reutilización**
Otros proyectos pueden usar solo lo que necesiten:

```toml
# Otro proyecto puede usar solo el dominio:
[dependencies]
reservas-domain = { git = "...", version = "0.1" }
```

### 4. **Versionado Independiente**
Cada crate puede tener su propia versión:
```toml
reservas-domain = "2.0.0"      # Nueva versión con breaking changes
reservas-adapters = "1.5.0"    # Compatible con domain 1.x y 2.x
```

### 5. **Testing Aislado**
```bash
# Test solo del dominio (rápido, sin deps):
cargo test -p reservas-domain

# Test de un adaptador específico:
cargo test -p reservas-adapters

# Test de todo:
cargo test
```

## Comandos Útiles

### Compilar todo el workspace:
```bash
cargo build
```

### Compilar solo un crate:
```bash
cargo build -p reservas-domain
```

### Ejecutar el binario:
```bash
cargo run -p reservas-app
# o simplemente:
cargo run
```

### Ver el grafo de dependencias:
```bash
cargo tree -p reservas-app
```

### Compilar solo con cambios:
```bash
# Si solo cambias el dominio, cargo solo recompila:
# - domain
# - ports (depende de domain)
# - application (depende de ports)
# - adapters (depende de ports)
# - reservas-app (depende de todos)
```

## Reglas de Dependencia

### ✅ Permitido:
- `application` → `ports` → `domain`
- `adapters` → `ports` → `domain`
- `reservas-app` → cualquiera

### ❌ Prohibido (el compilador lo evita):
- `domain` → cualquier otro crate
- `ports` → `application` o `adapters`
- `application` ↔ `adapters` (circular)

## Siguiente Paso: Añadir PostgreSQL

Cuando queramos añadir un adaptador de PostgreSQL:

```bash
# Creamos un nuevo crate:
cargo new crates/adapters-postgres --lib

# En su Cargo.toml:
[dependencies]
reservas-domain = { path = "../domain" }
reservas-ports = { path = "../ports" }
sqlx = { version = "0.7", features = ["postgres"] }
```

Y el resto de la aplicación **no cambia nada**.

## Resumen

La separación en crates te da:

1. ✅ **Seguridad arquitectural** - El compilador es tu guardian
2. ✅ **Rendimiento** - Compilación paralela
3. ✅ **Modularidad** - Reutilización fácil
4. ✅ **Mantenibilidad** - Cambios localizados
5. ✅ **Testing** - Pruebas independientes y rápidas
