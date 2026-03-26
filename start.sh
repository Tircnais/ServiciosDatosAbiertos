#!/bin/bash
export DB_HOST=0.0.0.0
export DB_PORT=8080

if [ ! -f "target/debug/ServiciosDatosAbiertos" ]; then
    echo "Compilando..."
    cargo build
fi

./target/debug/ServiciosDatosAbiertos
