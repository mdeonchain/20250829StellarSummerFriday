# Configuración de entorno de desarrollo para Stellar CLI en Ubuntu (Bash)

# 1. Actualizar repositorios e instalar dependencias básicas
sudo apt-get update
sudo apt-get install -y curl build-essential

# 2. Instalar Rust mediante rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. Configurar variable de entorno para que rustup sea reconocible en Bash
echo export "$HOME/.cargo/env"


# 4. Añadir soporte para el target wasm32v1-none
rustup target add wasm32v1-none

# 5. Descargar e instalar Stellar CLI
wget https://github.com/stellar/stellar-cli/releases/download/v23.0.1/stellar-cli-23.0.1-x86_64-unknown-linux-gnu.tar.gz
tar -xvf stellar-cli-23.0.1-x86_64-unknown-linux-gnu.tar.gz
sudo mv stellar /usr/local/bin/
sudo chmod +x /usr/local/bin/stellar

# 6. Verificar instalaciones
echo "=== Verificando instalación de Rust ==="
rustc --version
cargo --version
rustup --version

echo "=== Verificando instalación de Stellar CLI ==="
stellar --version

echo "Instalación completada correctamente."