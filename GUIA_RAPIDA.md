# Guía Rápida - Arquitectura Hexagonal Multi-Crate

## 🎯 ¿Qué acabas de construir?

Un sistema de reservas con **4 crates independientes** que implementan arquitectura hexagonal.

## 📦 Los 4 Crates

```
┌─────────────────────────────────────────────────┐
│  1. DOMAIN (crates/domain/)                     │
│  ⭐ El núcleo - SIN dependencias externas       │
│                                                  │
│  • Reserva (entidad)                            │
│  • EstadoReserva (enum)                         │
│  • ReservaError (errores de negocio)           │
│  • Validaciones (1-10 personas, fecha futura)  │
└─────────────────────────────────────────────────┘
                      ▲
                      │
┌─────────────────────┴───────────────────────────┐
│  2. PORTS (crates/ports/)                       │
│  🔌 Interfaces - Depende: domain                │
│                                                  │
│  • ReservaService (puerto entrada)              │
│  • ReservaRepository (puerto salida)            │
└─────────────────────────────────────────────────┘
                      ▲
              ┌───────┴────────┐
              │                │
┌─────────────┴──────┐  ┌──────┴──────────────────┐
│  3. APPLICATION    │  │  4. ADAPTERS            │
│  ⚙️ Casos de uso   │  │  🔧 Implementaciones    │
│  Depende: ports    │  │  Depende: ports         │
│                    │  │                         │
│  • ReservaService  │  │  • InMemoryRepository   │
│    Impl            │  │  • (Futuro) Postgres    │
└────────────────────┘  └─────────────────────────┘
```

## 🚀 Comandos Esenciales

```bash
# Ejecutar
cargo run

# Tests
cargo test

# Test de un crate específico
cargo test -p reservas-domain

# Ver dependencias
cargo tree -p reservas-app

# Compilar todo
cargo build
```

## 🎓 Reglas de Oro

### ✅ PERMITIDO:
```rust
// application depende de ports
use reservas_ports::ReservaService;

// ports depende de domain
use reservas_domain::Reserva;

// adapters depende de ports
use reservas_ports::ReservaRepository;
```

### ❌ PROHIBIDO (el compilador lo evita):
```rust
// ❌ domain NO puede depender de NADA
// ❌ ports NO puede depender de application
// ❌ ports NO puede depender de adapters
```

## 💡 ¿Por qué multi-crate?

### Antes (single crate):
```
src/
├── domain/
├── ports/
├── application/
└── adapters/

❌ Nada impide que domain importe de adapters
❌ Fácil romper la arquitectura accidentalmente
```

### Ahora (multi-crate):
```
crates/
├── domain/      (crate independiente)
├── ports/       (crate con deps explícitas)
├── application/ (crate con deps explícitas)
└── adapters/    (crate con deps explícitas)

✅ El compilador FUERZA las dependencias correctas
✅ Imposible romper la arquitectura
```

## 🔄 Flujo Completo de una Operación

```rust
// 1. Usuario llama (main.rs)
service.crear_reserva("Juan", fecha, 4).await?

// 2. Application (ReservaServiceImpl)
let id = Uuid::new_v4();
let reserva = Reserva::new(id, nombre, fecha, num)?  // ← llama al dominio

// 3. Domain (Reserva::new)
if num_personas > 10 { return Err(...) }  // ← valida reglas
if fecha < Utc::now() { return Err(...) }

// 4. Application guarda
self.repository.guardar(&reserva).await?  // ← usa el puerto

// 5. Adapter (InMemoryRepository)
storage.insert(id, reserva);  // ← implementación concreta
```

## 🎯 Ventajas Clave

| Ventaja | Explicación |
|---------|-------------|
| **Seguridad** | El compilador evita dependencias inválidas |
| **Testing** | Cada crate se prueba independientemente |
| **Velocidad** | Compilación paralela de crates |
| **Reutilización** | Otros proyectos pueden usar solo `domain` |
| **Mantenibilidad** | Cambios localizados en cada crate |

## 📊 Grafo de Compilación

```
cargo build ejecuta:

1. reservas-domain     ← Sin deps, compila primero
         │
         ▼
2. reservas-ports      ← Espera domain
         │
    ┌────┴────┐
    ▼         ▼
3. application + adapters  ← Compilan en PARALELO
         │
         ▼
4. reservas-app        ← Compila al final
```

## 🔨 Añadir Nueva Funcionalidad

### Ejemplo: Añadir PostgreSQL

```bash
# 1. Crear nuevo crate de adaptador
cd crates/adapters
# Editar src/postgres_repository.rs

# 2. Implementar el puerto
impl ReservaRepository for PostgresRepository {
    // ... implementación
}

# 3. Usar en main.rs
let repo = PostgresRepository::new(db_url);
let service = ReservaServiceImpl::new(repo);
```

**El dominio NO cambia** ✨

## 📚 Siguiente Nivel

1. **API REST** - Añadir adaptador de entrada con Axum
2. **PostgreSQL** - Añadir adaptador de salida con SQLx
3. **CLI** - Añadir otro adaptador de entrada
4. **Eventos** - Domain Events para notificaciones

## 🎓 Conceptos Importantes

### Inversión de Dependencias
```
Tradicional:
Application → Database (depende de implementación)

Hexagonal:
Application → Port (interfaz) ← Database (implementa interfaz)
```

### Puertos vs Adaptadores
- **Puerto** = Interfaz (trait en Rust)
- **Adaptador** = Implementación concreta

### Entrada vs Salida
- **Puerto Entrada** = Cómo USAR el sistema (`ReservaService`)
- **Puerto Salida** = Qué NECESITA el sistema (`ReservaRepository`)

## ✅ Checklist de Aprendizaje

- [x] Entiendes qué es un crate
- [x] Entiendes el workspace de Cargo
- [x] Sabes por qué el dominio no tiene dependencias
- [x] Entiendes qué son los puertos
- [x] Entiendes qué son los adaptadores
- [x] Puedes agregar un nuevo test
- [ ] Puedes agregar un nuevo adaptador (PostgreSQL)
- [ ] Puedes agregar una API REST

## 🚀 ¡Listo!

Ya tienes un sistema completo con arquitectura hexagonal multi-crate.

**Prueba esto:**
```bash
# ¿Qué pasa si intentas añadir esto a domain/src/lib.rs?
use reservas_ports::ReservaService;  # ← Intenta compilar

# Respuesta: ERROR! El compilador te lo impide 🎉
```
