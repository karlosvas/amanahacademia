FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --bin amanahacademia

# Runtime stage optimizada
FROM debian:bookworm-slim AS runtime

# Crear usuario no-root para seguridad
RUN useradd -r -s /bin/false amanahapp

# Instalar dependencias y limpiar en una sola capa
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Crear directorio y copiar binario
WORKDIR /app
COPY --from=builder /app/target/release/amanahacademia ./amanahacademia

# Cambiar permisos y propietario
RUN chown amanahapp:amanahapp /app/amanahacademia && \
    chmod +x /app/amanahacademia

# Cambiar a usuario no-root
USER amanahapp

# Usar EXPOSE si tu app tiene un puerto específico
EXPOSE 3000

# Usar CMD en lugar de ENTRYPOINT para mayor flexibilidad
CMD ["./amanahacademia"]