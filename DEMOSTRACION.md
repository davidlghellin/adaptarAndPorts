# Demostración: El Compilador Protege tu Arquitectura

## 🧪 Experimento 1: Intentar romper las dependencias

### Intenta esto (¡va a fallar!):

Abre [crates/domain/src/lib.rs](crates/domain/src/lib.rs) y añade al principio:

```rust
use reservas_ports::ReservaService;  // ← Intenta añadir esto
```

Luego ejecuta:
```bash
cargo build -p reservas-domain
```

### ❌ Resultado:
```
error[E0432]: unresolved import `reservas_ports`
 --> crates/domain/src/lib.rs:1:5
  |
1 | use reservas_ports::ReservaService;
  |     ^^^^^^^^^^^^^^ maybe a missing crate `reservas_ports`?
```

### ✅ ¿Por qué?
El `Cargo.toml` de domain NO incluye `reservas-ports` en sus dependencias:

```toml
# crates/domain/Cargo.toml
[dependencies]
chrono = { workspace = true }
# ← No hay reservas-ports, así que no puedes importarlo!
```

**El compilador te protege de violar la arquitectura** 🛡️

---

## 🧪 Experimento 2: Cambiar de InMemory a otra implementación

Vamos a demostrar que podemos cambiar el adaptador SIN tocar el dominio.

### Crea un "FakeRepository" para tests:

```rust
// En crates/adapters/src/fake_repository.rs
use reservas_domain::Reserva;
use reservas_ports::ReservaRepository;
use async_trait::async_trait;

pub struct FakeRepository;

#[async_trait]
impl ReservaRepository for FakeRepository {
    async fn guardar(&self, _: &Reserva) -> Result<(), String> {
        println!("FAKE: Guardando en fake storage");
        Ok(())
    }

    async fn obtener(&self, _: &str) -> Result<Option<Reserva>, String> {
        println!("FAKE: Obteniendo de fake storage");
        Ok(None)
    }

    // ... implementar el resto
}
```

### Usa el fake en main.rs:

```rust
// Antes:
let repository = InMemoryReservaRepository::new();

// Después:
let repository = FakeRepository;

let service = ReservaServiceImpl::new(repository);
// ← El servicio NO SABE ni LE IMPORTA qué repositorio es!
```

### ✅ Resultado:
La aplicación funciona igual, pero ahora usa el FakeRepository.

**El dominio NO cambió. Application NO cambió. Solo cambiamos el adaptador.**

---

## 🧪 Experimento 3: Compilación incremental

### Paso 1: Compila todo
```bash
cargo build
# Compila los 4 crates
```

### Paso 2: Modifica SOLO el dominio
```rust
// En crates/domain/src/reserva.rs
// Cambia el límite de personas de 10 a 8
if num_personas == 0 || num_personas > 8 {  // ← cambio aquí
    return Err(ReservaError::NumeroPersonasInvalido);
}
```

### Paso 3: Recompila
```bash
cargo build
```

### ✅ Observa:
```
Compiling reservas-domain v0.1.0
Compiling reservas-ports v0.1.0      ← Recompila (depende de domain)
Compiling reservas-application v0.1.0 ← Recompila (depende de ports)
Compiling reservas-adapters v0.1.0    ← Recompila (depende de ports)
Compiling reservas-app v0.1.0         ← Recompila (depende de todos)
```

Cargo **automáticamente** recompila solo lo necesario siguiendo el grafo de dependencias.

---

## 🧪 Experimento 4: Tests independientes

### Prueba cada capa por separado:

```bash
# Solo dominio (muy rápido, sin deps):
cargo test -p reservas-domain
# → Tarda ~0.2s

# Solo adaptadores:
cargo test -p reservas-adapters
# → Tarda ~0.3s

# Todo:
cargo test
# → Tarda un poco más
```

### ✅ Ventaja:
En CI/CD puedes ejecutar tests en paralelo:
```yaml
# GitHub Actions
jobs:
  test-domain:
    run: cargo test -p reservas-domain
  test-adapters:
    run: cargo test -p reservas-adapters
  # ← Se ejecutan en paralelo!
```

---

## 🧪 Experimento 5: Reutilización

Imagina que quieres usar el dominio en otro proyecto:

### Proyecto A (nuestro sistema):
```toml
# reservas-app/Cargo.toml
[dependencies]
reservas-domain = { path = "../crates/domain" }
reservas-adapters = { path = "../crates/adapters" }
```

### Proyecto B (un sistema diferente):
```toml
# otro-proyecto/Cargo.toml
[dependencies]
# Solo usa el dominio, sin adapters!
reservas-domain = { git = "https://...", version = "1.0" }
```

El Proyecto B puede usar la entidad `Reserva` y sus validaciones SIN arrastrar todo el sistema.

---

## 🧪 Experimento 6: Dependencias circulares imposibles

### Intenta esto:

1. En `crates/domain/Cargo.toml`, añade:
```toml
[dependencies]
reservas-ports = { path = "../ports" }
```

2. Ahora `ports` ya depende de `domain`, así que tenemos:
```
domain → ports
ports → domain
```

3. Ejecuta:
```bash
cargo build
```

### ❌ Resultado:
```
error: cyclic package dependency:
package `reservas-domain v0.1.0` depends on itself.
Cycle:
  reservas-domain v0.1.0
  └─ reservas-ports v0.1.0
     └─ reservas-domain v0.1.0
```

**Cargo detecta y previene dependencias circulares** 🛡️

---

## 📊 Resumen de Protecciones

| Protección | Mecanismo | Beneficio |
|------------|-----------|-----------|
| **Dependencias unidireccionales** | Cargo.toml | El dominio nunca depende de infra |
| **Sin circulares** | Cargo resolver | Evita ciclos de dependencia |
| **Compilación incremental** | Cargo build | Solo recompila lo necesario |
| **Tests aislados** | Crates separados | Tests rápidos y focalizados |
| **Versionado** | Semver por crate | Evolución independiente |

---

## 🎯 Conclusión

La separación en crates no es solo organizacional - **es seguridad arquitectural**:

✅ El compilador es tu guardián
✅ Imposible violar las reglas sin querer
✅ Errores en tiempo de compilación, no de ejecución
✅ Refactorización segura

**La arquitectura está garantizada por el sistema de tipos de Rust** 🦀
