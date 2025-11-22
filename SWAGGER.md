# Documentación API con Swagger/OpenAPI

Este proyecto incluye documentación automática de la API REST usando **Swagger UI** y **OpenAPI 3.1**.

## Acceder a la documentación

Una vez que el servidor esté corriendo (`cargo run -p api-server`), puedes acceder a:

### Swagger UI (Interfaz interactiva)
```
http://localhost:3000/swagger-ui
```

La interfaz de Swagger UI te permite:
- Ver todos los endpoints disponibles organizados por tags
- Ver los esquemas de request/response con ejemplos
- **Probar los endpoints directamente desde el navegador**
- Ver códigos de estado HTTP y descripciones de errores

### OpenAPI JSON (Especificación)
```
http://localhost:3000/api-docs/openapi.json
```

Este endpoint devuelve la especificación completa de la API en formato OpenAPI 3.1 JSON.

## Características de la documentación

- **Esquemas completos**: Todos los DTOs están documentados con ejemplos
- **Parámetros**: Query params y path params con descripciones
- **Respuestas**: Códigos HTTP con sus respectivos schemas
- **Tags organizados**:
  - `Empleados`: Gestión de empleados
  - `Reservas`: Gestión de reservas de tiempo
  - `Disponibilidad`: Consulta de disponibilidad

## Probar desde Swagger UI

1. Abre `http://localhost:3000/swagger-ui`
2. Expande cualquier endpoint (ej: `POST /empleados`)
3. Haz click en "Try it out"
4. Edita el JSON de ejemplo si es necesario
5. Haz click en "Execute"
6. Verás la respuesta del servidor en tiempo real

## Ejemplo: Crear un empleado desde Swagger

1. Expande `POST /empleados`
2. Click "Try it out"
3. El JSON de ejemplo ya está pre-cargado:
```json
{
  "nombre": "Juan López",
  "email": "juan@empresa.com"
}
```
4. Click "Execute"
5. Verás la respuesta con el ID generado

¡Así de fácil! 🎉
