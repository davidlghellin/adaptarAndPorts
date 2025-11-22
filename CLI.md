# CLI del Sistema de Reservas

Este proyecto incluye un CLI (Command Line Interface) completo para interactuar con el sistema de reservas desde la terminal.

## Compilar y ejecutar

```bash
# Compilar
cargo build -p cli-app

# Ejecutar (usando el nombre del paquete)
cargo run -p cli-app -- [COMANDO]

# O usar el nombre del binario directamente
cargo run --bin reservas -- [COMANDO]
```

## Arquitectura

El CLI es un **ADAPTADOR DE ENTRADA** que sigue la arquitectura hexagonal:

```
CLI (Adaptador de Entrada)
    ↓
HTTP Client (reqwest)
    ↓
API REST (Puerto de entrada)
    ↓
Servicios de Aplicación
    ↓
Dominio
```

## Comandos disponibles

### Ver ayuda

```bash
cargo run -p cli-app -- --help
```

### Gestión de Empleados

**Crear empleado:**
```bash
cargo run -p cli-app -- empleado crear \
  --nombre "Juan López" \
  --email "juan@empresa.com"
```

**Listar empleados:**
```bash
cargo run -p cli-app -- empleado listar
```

**Obtener información de un empleado:**
```bash
cargo run -p cli-app -- empleado obtener --id <ID>
```

**Activar/Desactivar empleado:**
```bash
cargo run -p cli-app -- empleado activar --id <ID>
cargo run -p cli-app -- empleado desactivar --id <ID>
```

### Gestión de Reservas

**Crear reserva:**
```bash
cargo run -p cli-app -- reserva crear \
  --empleado-id <ID> \
  --fecha "2025-11-25" \
  --hora 10 \
  --descripcion "Reunión importante"
```

**Listar todas las reservas:**
```bash
cargo run -p cli-app -- reserva listar
```

**Listar reservas de un empleado:**
```bash
cargo run -p cli-app -- reserva listar-empleado --empleado-id <ID>
```

**Confirmar reserva:**
```bash
cargo run -p cli-app -- reserva confirmar --id <ID>
```

**Cancelar reserva:**
```bash
cargo run -p cli-app -- reserva cancelar --id <ID>
```

### Ver Disponibilidad

**Ver disponibilidad para una fecha:**
```bash
cargo run -p cli-app -- disponibilidad --fecha "2025-11-25"
```

Esto muestra:
- Todos los slots disponibles del día
- Estado de cada empleado en cada slot
- Descripción de las reservas ocupadas

## Configuración

### URL del servidor

Por defecto, el CLI se conecta a `http://localhost:3000`. Para usar otro servidor:

```bash
cargo run -p cli-app -- --url http://otro-servidor:8080 empleado listar
```

## Características

- **Colores**: Usa colores para distinguir éxitos ✓ (verde) y errores ✗ (rojo)
- **Tablas**: Muestra datos en formato tabla para mejor legibilidad
- **Errores claros**: Mensajes de error descriptivos
- **Ayuda integrada**: Cada comando tiene `--help`

## Ejemplo de flujo completo

```bash
# 1. Crear empleados
cargo run -p cli-app -- empleado crear --nombre "Juan" --email "juan@empresa.com"
cargo run -p cli-app -- empleado crear --nombre "María" --email "maria@empresa.com"

# 2. Ver lista de empleados (copiar el ID que necesites)
cargo run -p cli-app -- empleado listar

# 3. Crear una reserva para Juan
cargo run -p cli-app -- reserva crear \
  --empleado-id <ID-DE-JUAN> \
  --fecha "2025-11-25" \
  --hora 10 \
  --descripcion "Reunión con cliente"

# 4. Ver disponibilidad
cargo run -p cli-app -- disponibilidad --fecha "2025-11-25"
```

## Dependencias

- **clap**: Framework para CLIs con derivación de comandos
- **reqwest**: Cliente HTTP (blocking mode)
- **colored**: Colores en terminal
- **tabled**: Generación de tablas ASCII
- **chrono**: Manejo de fechas y horas
- **serde/serde_json**: Serialización JSON

## Estructura del código

La organización sigue la arquitectura hexagonal:

```
crates/cli/              # Librería del CLI (adaptador de entrada)
├── src/
│   ├── lib.rs          # Punto de entrada de la librería
│   ├── cli_args.rs     # Definición de comandos con clap
│   ├── api_client.rs   # Cliente HTTP para la API
│   └── commands.rs     # Implementación de cada comando
└── Cargo.toml

cli-app/                 # Binario ejecutable (punto de entrada)
├── src/
│   └── main.rs         # Thin wrapper que usa crates/cli
└── Cargo.toml
```

**Ventajas de esta organización:**

1. **Reutilización**: `crates/cli` se puede usar como librería en otros proyectos
2. **Consistencia**: Todos los adaptadores están en `crates/`
3. **Separación**: Binarios en raíz, librerías en `crates/`
4. **Testing**: El CLI se puede testear importándolo como librería

Cada archivo tiene una responsabilidad clara siguiendo el principio de responsabilidad única.
