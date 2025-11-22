# Arquitectura Hexagonal - Sistema de Reservas

## Estructura Visual

```
┌─────────────────────────────────────────────────────────────┐
│                    ADAPTADORES DE ENTRADA                    │
│                     (Futuros: API REST, CLI)                 │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    PUERTOS DE ENTRADA                        │
│                  (ReservaService trait)                      │
│                                                              │
│  - crear_reserva()                                           │
│  - obtener_reserva()                                         │
│  - listar_reservas()                                         │
│  - confirmar_reserva()                                       │
│  - cancelar_reserva()                                        │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                      APLICACIÓN                              │
│              (ReservaServiceImpl)                            │
│                                                              │
│  Orquesta la lógica de negocio                               │
│  y coordina el dominio con los puertos                       │
└──────────┬────────────────────────────────┬─────────────────┘
           │                                │
           ▼                                ▼
┌──────────────────────┐        ┌──────────────────────────────┐
│      DOMINIO         │        │   PUERTOS DE SALIDA          │
│                      │        │  (ReservaRepository trait)   │
│  - Reserva           │        │                              │
│  - EstadoReserva     │        │  - guardar()                 │
│  - ReservaError      │        │  - obtener()                 │
│                      │        │  - listar()                  │
│  Lógica de negocio   │        │  - actualizar()              │
│  pura sin            │        │  - existe()                  │
│  dependencias        │        └──────────┬───────────────────┘
└──────────────────────┘                   │
                                           ▼
                            ┌──────────────────────────────────┐
                            │   ADAPTADORES DE SALIDA          │
                            │                                  │
                            │  - InMemoryRepository (actual)   │
                            │  - PostgresRepository (futuro)   │
                            │  - MongoRepository (futuro)      │
                            └──────────────────────────────────┘
```

## Capas Explicadas

### 1. **DOMINIO** (`src/domain/`)
- **¿Qué es?** El corazón de tu aplicación
- **Contiene:** Entidades, reglas de negocio, validaciones
- **Independiente de:** Bases de datos, frameworks, APIs
- **Ejemplo:** La entidad `Reserva` con sus validaciones (1-10 personas, fecha futura)

### 2. **PUERTOS** (`src/ports/`)
- **¿Qué son?** Interfaces (traits en Rust) que definen contratos
- **Dos tipos:**
  - **Entrada:** Cómo usar el sistema (`ReservaService`)
  - **Salida:** Qué necesita el sistema (`ReservaRepository`)
- **Ventaja:** Puedes cambiar implementaciones sin romper nada

### 3. **APLICACIÓN** (`src/application/`)
- **¿Qué es?** Los casos de uso de tu sistema
- **Contiene:** `ReservaServiceImpl` que implementa `ReservaService`
- **Función:** Orquesta el dominio usando los repositorios

### 4. **ADAPTADORES** (`src/adapters/`)
- **¿Qué son?** Implementaciones concretas de los puertos
- **Tipos:**
  - **Entrada:** API REST, CLI, GraphQL (futuros)
  - **Salida:** InMemory, Postgres, MongoDB
- **Ventaja:** Se pueden cambiar sin tocar el núcleo

## Flujo de una Operación

```
1. Usuario llama → service.crear_reserva()
                   ↓
2. ReservaServiceImpl (aplicación)
   - Genera ID con UUID
   - Llama a Reserva::new() (dominio valida)
   - Verifica si existe
                   ↓
3. Dominio valida las reglas de negocio
   - ¿Número de personas válido?
   - ¿Fecha en el futuro?
                   ↓
4. Si es válido, guarda usando el puerto
   repository.guardar(&reserva)
                   ↓
5. InMemoryRepository (adaptador)
   - Guarda en HashMap
```

## Ventajas Clave

✅ **Testeable:** Cada capa se prueba independientemente

✅ **Flexible:** Cambias de InMemory a Postgres sin tocar el dominio

✅ **Mantenible:** Cambios en UI no afectan la lógica de negocio

✅ **Comprensible:** Separación clara de responsabilidades

## Próximos Pasos

1. **Añadir API REST** (adaptador de entrada con Axum/Actix)
2. **Añadir PostgreSQL** (adaptador de salida)
3. **Más lógica de dominio** (evitar solapamientos, capacidad máxima)
4. **Event sourcing** (registrar eventos de dominio)
