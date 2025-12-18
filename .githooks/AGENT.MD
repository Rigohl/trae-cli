---
description: "Contexto .githooks - Git Hooks Personalizados"
agent: "Git_Hooks"
capabilities: ["pre_commit_validation", "code_quality_enforcement", "automated_testing"]
triggers: ["git_commit", "git_push", "code_changes"]
---

# .GITHOOKS - CONTEXTO DE CARPETA

## DESCRIPCIÓN GENERAL
Esta carpeta contiene hooks de Git personalizados para TRAE-CLI. Implementa validación automática de código, testing y cumplimiento de estándares antes de commits y pushes.

## ARQUITECTURA DE GIT HOOKS

### Tipos de Hooks Implementados

#### Pre-commit Hook
- **Validación de Código**: Ejecuta `cargo check` y `cargo clippy`
- **Formateo**: Asegura que `cargo fmt` ha sido ejecutado
- **Tests**: Corre tests unitarios básicos
- **Zero Warnings**: Falla si hay warnings de compilación

#### Pre-push Hook
- **Tests Completos**: Ejecuta `cargo test --all-targets`
- **Integration Tests**: Corre tests de integración si existen
- **Benchmark Checks**: Verifica que benchmarks compilen
- **Documentation**: Valida que docs se generen correctamente

#### Commit-msg Hook
- **Formato de Mensajes**: Valida formato de commit messages
- **Issue References**: Verifica referencias a issues
- **Conventional Commits**: Enforce conventional commit format

### Estructura de Hooks
```
.githooks/
├── pre-commit          # Validación antes de commit
├── pre-push           # Validación antes de push
├── commit-msg         # Validación de mensaje de commit
├── post-commit        # Acciones post-commit
├── post-merge         # Acciones post-merge
└── prepare-commit-msg # Preparación de mensaje de commit
```

## PRE-COMMIT HOOK

### Implementación Principal
```bash
#!/bin/bash
# .githooks/pre-commit

set -e  # Exit on any error

echo "🚀 Running TRAE-CLI pre-commit hooks..."

# 1. Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust."
    exit 1
fi

# 2. Run cargo check (fast compilation check)
echo "🔍 Running cargo check..."
if ! cargo check --quiet; then
    echo "❌ Cargo check failed. Please fix compilation errors."
    echo "💡 Run 'cargo check' to see detailed errors."
    exit 1
fi

# 3. Run cargo clippy (linting)
echo "🔧 Running cargo clippy..."
if ! cargo clippy --quiet -- -D warnings; then
    echo "❌ Clippy found warnings. Please fix them."
    echo "💡 Run 'cargo clippy' to see detailed warnings."
    exit 1
fi

# 4. Check formatting
echo "📝 Checking code formatting..."
if ! cargo fmt --check; then
    echo "❌ Code is not properly formatted."
    echo "💡 Run 'cargo fmt' to format your code."
    exit 1
fi

# 5. Run basic tests
echo "🧪 Running unit tests..."
if ! cargo test --quiet --lib; then
    echo "❌ Unit tests failed. Please fix them."
    echo "💡 Run 'cargo test' to see failing tests."
    exit 1
fi

echo "✅ All pre-commit checks passed!"
exit 0
```

### Configuración de Clippy
```bash
# Configuración adicional para clippy en pre-commit
export RUSTFLAGS="-D warnings"
cargo clippy -- \
    -D clippy::all \
    -D clippy::pedantic \
    -D clippy::nursery \
    --deny warnings
```

## PRE-PUSH HOOK

### Validaciones Avanzadas
```bash
#!/bin/bash
# .githooks/pre-push

set -e

echo "🚀 Running TRAE-CLI pre-push validation..."

# 1. Run full test suite
echo "🧪 Running full test suite..."
if ! cargo test --all-targets --quiet; then
    echo "❌ Full test suite failed."
    echo "💡 Run 'cargo test --all-targets' to debug."
    exit 1
fi

# 2. Check documentation
echo "📚 Checking documentation..."
if ! cargo doc --no-deps --quiet; then
    echo "❌ Documentation generation failed."
    echo "💡 Run 'cargo doc' to see issues."
    exit 1
fi

# 3. Run benchmarks (if they exist)
if cargo bench --help &> /dev/null; then
    echo "⚡ Running benchmarks..."
    if ! cargo bench --quiet; then
        echo "❌ Benchmarks failed."
        echo "💡 Run 'cargo bench' to debug."
        exit 1
    fi
fi

# 4. Check for security issues
if command -v cargo-audit &> /dev/null; then
    echo "🔒 Running security audit..."
    if ! cargo audit --quiet; then
        echo "❌ Security audit found issues."
        echo "💡 Run 'cargo audit' to see vulnerabilities."
        exit 1
    fi
fi

# 5. Validate CI configuration
echo "🔄 Validating CI configuration..."
if [ -f ".github/workflows/ci.yml" ]; then
    if ! yamllint .github/workflows/ci.yml; then
        echo "❌ CI configuration has YAML issues."
        exit 1
    fi
fi

echo "✅ All pre-push validations passed!"
exit 0
```

## COMMIT-MSG HOOK

### Validación de Mensajes
```bash
#!/bin/bash
# .githooks/commit-msg

set -e

COMMIT_MSG_FILE="$1"
COMMIT_MSG=$(cat "$COMMIT_MSG_FILE")

echo "📝 Validating commit message..."

# 1. Check minimum length
if [ ${#COMMIT_MSG} -lt 10 ]; then
    echo "❌ Commit message too short (minimum 10 characters)."
    exit 1
fi

# 2. Check conventional commit format
if ! echo "$COMMIT_MSG" | grep -qE "^(feat|fix|docs|style|refactor|test|chore)(\(.+\))?: .{10,}"; then
    echo "❌ Commit message doesn't follow conventional commit format."
    echo "💡 Use: type(scope): description"
    echo "     Types: feat, fix, docs, style, refactor, test, chore"
    exit 1
fi

# 3. Check for issue references
if ! echo "$COMMIT_MSG" | grep -qE "#[0-9]+|closes #[0-9]+|fixes #[0-9]+"; then
    echo "⚠️  Consider referencing an issue (e.g., #123, closes #123)."
fi

echo "✅ Commit message validation passed!"
exit 0
```

## POST-COMMIT HOOK

### Acciones Automáticas
```bash
#!/bin/bash
# .githooks/post-commit

echo "📊 Running post-commit actions..."

# 1. Update changelog if needed
if [ -f "CHANGELOG.md" ]; then
    echo "📝 Updating changelog..."
    # Logic to update changelog based on commit
fi

# 2. Run light tests to ensure everything still works
echo "🔍 Running quick health check..."
cargo check --quiet

# 3. Update any generated files
echo "🔄 Updating generated files..."
cargo run --bin generate-docs 2>/dev/null || true

echo "✅ Post-commit actions completed!"
```

## CONFIGURACIÓN E INSTALACIÓN

### Instalación Automática
```bash
#!/bin/bash
# scripts/install-hooks.sh

echo "🔧 Installing TRAE-CLI git hooks..."

# Set git hooks path
git config core.hooksPath .githooks

# Make hooks executable
chmod +x .githooks/*

echo "✅ Git hooks installed successfully!"
echo "💡 Hooks will run automatically on commit/push."
```

### Configuración Global
```bash
# .githooks/config - Configuración de hooks
[hooks]
    enable_pre_commit = true
    enable_pre_push = true
    enable_commit_msg = true
    strict_mode = true
    allow_warnings = false
```

## INTEGRACIÓN CON TRAE-CLI

### Comandos TRAE-CLI
```bash
# Verificar hooks
trae hooks --status

# Instalar hooks
trae hooks --install

# Ejecutar validaciones manualmente
trae validate --pre-commit
trae validate --pre-push
```

### Configuración TRAE-CLI
```toml
# Cargo.toml - Configuración de hooks
[package.metadata.trae.hooks]
pre_commit = ["cargo check", "cargo clippy", "cargo fmt --check", "cargo test"]
pre_push = ["cargo test --all-targets", "cargo audit", "cargo doc"]
commit_msg = ["conventional-commits"]
```

## TESTING DE HOOKS

### Tests Unitarios
```rust
// tests/hooks_test.rs
#[test]
fn test_pre_commit_hook() {
    // Setup test repository
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    // Configure hooks
    std::fs::create_dir_all(temp_dir.path().join(".githooks")).unwrap();
    std::fs::copy("pre-commit", temp_dir.path().join(".githooks/pre-commit")).unwrap();

    // Make test commit
    std::fs::write(temp_dir.path().join("test.rs"), "fn main() {}").unwrap();
    repo.index().unwrap().add_path(Path::new("test.rs")).unwrap();

    // Test hook execution
    let output = Command::new("git")
        .args(&["commit", "-m", "test commit"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    assert!(output.status.success());
}
```

## MANEJO DE ERRORES

### Recuperación de Hooks
```bash
# Saltar hooks temporalmente
git commit --no-verify -m "fix: urgent fix"

# Ejecutar hooks manualmente
./.githooks/pre-commit
./.githooks/pre-push
```

### Debugging de Hooks
```bash
# Ejecutar con debug
GIT_TRACE=1 git commit -m "test"

# Ver logs detallados
export TRAE_HOOKS_DEBUG=1
git commit -m "test"
```

## CONTEXTO DE DESARROLLO

Los git hooks garantizan la calidad del código en TRAE-CLI. Implementan el principio "Zero Warnings" y "No Mocks" al nivel de control de versiones, previniendo commits problemáticos.

### Puntos de Contacto
- **Git Workflow**: Se ejecutan automáticamente en operaciones git
- **CI/CD**: Complementan las validaciones de GitHub Actions
- **TRAE-CLI**: Integran con comandos de análisis y validación

### Mejores Prácticas
- Mantener hooks ligeros para no ralentizar desarrollo
- Proporcionar mensajes de error claros y accionables
- Permitir bypass con --no-verify para casos de emergencia
- Documentar qué valida cada hook
- Actualizar hooks cuando cambien estándares del proyecto
