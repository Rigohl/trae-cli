---
description: "Contexto bin - Binarios Compilados y Ejecutables"
agent: "Binary_Distribution"
capabilities: ["executable_distribution", "cross_compilation", "release_artifacts"]
triggers: ["build_success", "release_creation", "binary_deployment"]
---

# BIN - CONTEXTO DE CARPETA

## DESCRIPCIÓN GENERAL
Esta carpeta contiene los binarios compilados de TRAE-CLI, incluyendo ejecutables para múltiples plataformas y configuraciones de distribución.

## ARQUITECTURA DE BINARIOS

### Tipos de Binarios

#### Binarios de Release
- **trae**: Ejecutable principal de TRAE-CLI
- **trae.exe**: Versión Windows
- **trae-macos**: Versión macOS
- **trae-linux**: Versión Linux

#### Binarios de Desarrollo
- **trae-debug**: Versión con símbolos de debug
- **trae-dev**: Versión de desarrollo con features adicionales

#### Binarios de Testing
- **trae-test**: Binario instrumentado para testing
- **trae-bench**: Binario optimizado para benchmarks

### Estructura de Distribución
```
bin/
├── linux/
│   ├── x86_64/
│   │   ├── trae
│   │   ├── trae.sha256
│   │   └── trae.tar.gz
│   └── aarch64/
│       ├── trae
│       └── trae.sha256
├── macos/
│   ├── x86_64/
│   │   ├── trae
│   │   └── trae.sha256
│   └── aarch64/
│       ├── trae
│       └── trae.sha256
├── windows/
│   ├── x86_64/
│   │   ├── trae.exe
│   │   └── trae.exe.sha256
│   └── aarch64/
│       ├── trae.exe
│       └── trae.exe.sha256
└── checksums.txt
```

## COMPILACIÓN CROSS-PLATFORM

### Configuración de Cross-Compilation
```toml
# Cross.toml - Configuración para cross-rs
[target.x86_64-unknown-linux-gnu]
image = "rustembedded/cross:x86_64-unknown-linux-gnu"

[target.aarch64-unknown-linux-gnu]
image = "rustembedded/cross:aarch64-unknown-linux-gnu"

[target.x86_64-apple-darwin]
image = "rustembedded/cross:x86_64-apple-darwin"

[target.aarch64-apple-darwin]
image = "rustembedded/cross:aarch64-apple-darwin"

[target.x86_64-pc-windows-gnu]
image = "rustembedded/cross:x86_64-pc-windows-gnu"
```

### Script de Build Multiplataforma
```bash
#!/bin/bash
# scripts/build-release.sh

set -e

TARGETS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-gnu"
)

echo "🚀 Building TRAE-CLI for multiple platforms..."

for target in "${TARGETS[@]}"; do
    echo "📦 Building for $target..."

    # Build with cross
    cross build --release --target $target

    # Determine output directory and binary name
    case $target in
        *-linux-*)
            bin_name="trae"
            platform="linux"
            ;;
        *-darwin)
            bin_name="trae"
            platform="macos"
            ;;
        *-windows-*)
            bin_name="trae.exe"
            platform="windows"
            ;;
    esac

    # Extract architecture
    arch=$(echo $target | cut -d'-' -f1)

    # Create output directory
    output_dir="bin/$platform/$arch"
    mkdir -p $output_dir

    # Copy binary
    cp "target/$target/release/$bin_name" "$output_dir/"

    # Generate checksum
    cd $output_dir
    sha256sum $bin_name > ${bin_name}.sha256
    cd - > /dev/null

    echo "✅ Built for $target"
done

echo "🎉 All platforms built successfully!"
```

## OPTIMIZACIONES DE COMPILACIÓN

### Release Profile Optimizado
```toml
# Cargo.toml - Perfil de release optimizado
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
debug = false

# Optimizaciones adicionales
[profile.release.package."*"]
opt-level = 3
```

### Link Time Optimization (LTO)
```bash
# Build con LTO completo
RUSTFLAGS="-C target-cpu=native" cargo build --release --config 'profile.release.lto="fat"'
```

### Binary Size Optimization
```bash
# Strip symbols para reducir tamaño
strip target/release/trae

# Comprimir con UPX
upx --best --lzma target/release/trae
```

## DISTRIBUCIÓN Y PAQUETES

### Paquetes .tar.gz
```bash
#!/bin/bash
# scripts/create-packages.sh

create_package() {
    local platform=$1
    local arch=$2
    local bin_name=$3

    echo "📦 Creating package for $platform-$arch..."

    cd bin/$platform/$arch

    # Create tarball
    tar -czf ${bin_name}.tar.gz $bin_name

    # Generate checksum for tarball
    sha256sum ${bin_name}.tar.gz > ${bin_name}.tar.gz.sha256

    cd - > /dev/null
}

# Create packages for all platforms
create_package "linux" "x86_64" "trae"
create_package "linux" "aarch64" "trae"
create_package "macos" "x86_64" "trae"
create_package "macos" "aarch64" "trae"
create_package "windows" "x86_64" "trae.exe"
```

### Checksums Globales
```bash
#!/bin/bash
# scripts/generate-checksums.sh

echo "🔐 Generating global checksums..."

cd bin

# Find all binaries and generate checksums
find . -name "trae*" -type f -not -name "*.sha256" -not -name "*.tar.gz" | while read file; do
    sha256sum "$file" >> checksums.txt
done

echo "✅ Checksums generated in bin/checksums.txt"
```

## INSTALACIÓN Y DEPLOYMENT

### Script de Instalación
```bash
#!/bin/bash
# install.sh

set -e

echo "🚀 Installing TRAE-CLI..."

# Detect platform and architecture
detect_platform() {
    case "$(uname -s)" in
        Linux*)     platform="linux" ;;
        Darwin*)    platform="macos" ;;
        CYGWIN*|MINGW*|MSYS*) platform="windows" ;;
        *)          echo "Unsupported platform"; exit 1 ;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        x86_64)     arch="x86_64" ;;
        aarch64)    arch="aarch64" ;;
        arm64)      arch="aarch64" ;;
        *)          echo "Unsupported architecture"; exit 1 ;;
    esac
}

detect_platform
detect_arch

# Download appropriate binary
binary_url="https://github.com/organization/trae-cli/releases/latest/download/trae-${platform}-${arch}.tar.gz"
install_dir="/usr/local/bin"

echo "📥 Downloading TRAE-CLI for $platform-$arch..."
curl -L -o trae.tar.gz "$binary_url"

# Extract and install
tar -xzf trae.tar.gz
sudo mv trae "$install_dir/"

# Verify installation
if command -v trae &> /dev/null; then
    echo "✅ TRAE-CLI installed successfully!"
    trae --version
else
    echo "❌ Installation failed"
    exit 1
fi
```

### Homebrew Formula
```ruby
# Formula/trae-cli.rb
class TraeCli < Formula
  desc "TRAE-CLI - Cargo extension for code analysis and repair"
  homepage "https://github.com/organization/trae-cli"
  url "https://github.com/organization/trae-cli/releases/download/v1.0.0/trae-macos-x86_64.tar.gz"
  sha256 "checksum_here"

  def install
    bin.install "trae"
  end

  test do
    system "#{bin}/trae", "--version"
  end
end
```

## TESTING DE BINARIOS

### Tests de Integración
```rust
// tests/binary_integration_test.rs
#[test]
fn test_binary_execution() {
    // Build binary
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .output()
        .unwrap();
    assert!(output.status.success());

    // Test binary execution
    let binary_path = "target/release/trae";
    let output = Command::new(binary_path)
        .arg("--version")
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("trae-cli"));
}

#[test]
fn test_cross_platform_binaries() {
    // Test that binaries exist for all platforms
    let platforms = ["linux", "macos", "windows"];
    let arches = ["x86_64", "aarch64"];

    for platform in &platforms {
        for arch in &arches {
            let bin_name = if *platform == "windows" { "trae.exe" } else { "trae" };
            let bin_path = format!("bin/{}/{}/{}", platform, arch, bin_name);
            assert!(Path::new(&bin_path).exists(), "Missing binary: {}", bin_path);
        }
    }
}
```

### Smoke Tests
```bash
#!/bin/bash
# scripts/smoke-test.sh

echo "🧪 Running smoke tests on binaries..."

platforms=("linux" "macos" "windows")
arches=("x86_64" "aarch64")

for platform in "${platforms[@]}"; do
    for arch in "${arches[@]}"; do
        echo "Testing $platform-$arch..."

        bin_name="trae"
        [ "$platform" = "windows" ] && bin_name="trae.exe"

        bin_path="bin/$platform/$arch/$bin_name"

        if [ ! -f "$bin_path" ]; then
            echo "❌ Binary not found: $bin_path"
            continue
        fi

        # Test basic functionality
        if "$bin_path" --version > /dev/null 2>&1; then
            echo "✅ $platform-$arch: version check passed"
        else
            echo "❌ $platform-$arch: version check failed"
        fi

        # Test help command
        if "$bin_path" --help > /dev/null 2>&1; then
            echo "✅ $platform-$arch: help command passed"
        else
            echo "❌ $platform-$arch: help command failed"
        fi
    done
done

echo "🎉 Smoke tests completed!"
```

## SEGURIDAD Y VERIFICACIÓN

### Verificación de Checksums
```bash
#!/bin/bash
# scripts/verify-checksums.sh

echo "🔐 Verifying binary checksums..."

cd bin

# Verify individual checksums
find . -name "*.sha256" | while read checksum_file; do
    binary_file="${checksum_file%.sha256}"

    if sha256sum -c "$checksum_file"; then
        echo "✅ $binary_file: checksum verified"
    else
        echo "❌ $binary_file: checksum verification failed"
        exit 1
    fi
done

echo "🎉 All checksums verified!"
```

### Firma de Binarios
```bash
# Firmar binarios con GPG
gpg --detach-sign --armor bin/linux/x86_64/trae

# Verificar firma
gpg --verify bin/linux/x86_64/trae.asc bin/linux/x86_64/trae
```

## INTEGRACIÓN CON TRAE-CLI

### Comandos de Distribución
```bash
# Construir todos los binarios
trae build --all-platforms

# Crear paquetes de distribución
trae package --create

# Verificar integridad
trae verify --checksums

# Subir a releases
trae release --upload
```

### Configuración de Build
```toml
# Cargo.toml - Configuración de distribución
[package.metadata.trae.distribution]
platforms = ["linux", "macos", "windows"]
architectures = ["x86_64", "aarch64"]
compression = "gzip"
signing = true
```

## CONTEXTO DE DESARROLLO

La carpeta bin/ contiene todos los artefactos de distribución de TRAE-CLI. Gestiona compilación multiplataforma, optimización de binarios y distribución segura.

### Puntos de Contacto
- **CI/CD**: Genera binarios automáticamente en releases
- **Cross-compilation**: Usa cross-rs para builds multiplataforma
- **Distribution**: Crea paquetes .tar.gz con checksums
- **Installation**: Scripts automatizados para instalación

### Mejores Prácticas
- Optimizar tamaño de binarios con stripping y compresión
- Generar checksums SHA256 para verificación
- Firmar binarios para distribución segura
- Probar binarios en todas las plataformas objetivo
- Mantener versiones específicas en subcarpetas
- Documentar proceso de instalación claramente
- Usar LTO para optimización máxima
- Implementar smoke tests post-build
