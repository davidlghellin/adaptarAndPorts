# Build stage
FROM rust:1.83 as builder

WORKDIR /app

# Copiar archivos de proyecto
COPY . .

# Compilar en modo release
RUN cargo build --release --bin api-server

# Runtime stage
FROM debian:bookworm-slim

# Instalar dependencias necesarias
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar el binario compilado
COPY --from=builder /app/target/release/api-server /usr/local/bin/api-server

# Copiar archivos estáticos y templates
COPY --from=builder /app/crates/web-ui/static /app/crates/web-ui/static
COPY --from=builder /app/crates/web-ui/templates /app/crates/web-ui/templates

EXPOSE 3000

CMD ["api-server"]
