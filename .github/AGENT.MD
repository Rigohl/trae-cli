---
description: "Contexto .github - GitHub Automation y Workflows"
agent: "GitHub_Automation"
capabilities: ["ci_cd", "code_quality", "release_management", "issue_tracking"]
triggers: ["pull_request", "push", "issue_opened", "release_created"]
---

# .GITHUB - CONTEXTO DE CARPETA

## DESCRIPCIÓN GENERAL
Esta carpeta contiene toda la automatización de GitHub para TRAE-CLI, incluyendo workflows de CI/CD, plantillas de issues/PRs, y configuraciones de dependabot.

## ARQUITECTURA DE GITHUB AUTOMATION

### Workflows CI/CD

#### CI Principal (ci.yml)
- **Triggers**: Push a main, PRs
- **Jobs**:
  - Test en múltiples plataformas (Linux, macOS, Windows)
  - Validación de código con clippy
  - Tests unitarios e integración
  - Coverage reporting
  - Release builds

#### Release Workflow (release.yml)
- **Triggers**: Tags de versión
- **Jobs**:
  - Build para múltiples targets
  - Crear releases en GitHub
  - Publicar en crates.io
  - Generar changelogs

#### Security Scan (security.yml)
- **Triggers**: Schedule semanal, dependabot PRs
- **Jobs**:
  - Cargo audit para vulnerabilidades
  - CodeQL analysis
  - Dependency scanning

### Estructura de Archivos
```
.github/
├── workflows/
│   ├── ci.yml              # CI principal
│   ├── release.yml         # Releases automáticos
│   ├── security.yml        # Escaneos de seguridad
│   └── dependabot.yml      # Actualizaciones de dependencias
├── ISSUE_TEMPLATE/
│   ├── bug_report.md       # Reporte de bugs
│   ├── feature_request.md  # Solicitud de features
│   └── security.md         # Reportes de seguridad
├── PULL_REQUEST_TEMPLATE.md # Template para PRs
├── CODEOWNERS             # Code owners
├── dependabot.yml         # Configuración dependabot
└── FUNDING.yml           # Configuración de funding
```

## CI WORKFLOW PRINCIPAL

### Configuración CI
```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]

    steps:
    - uses: actions/checkout@v4

    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        toolchain: ${{ matrix.rust }}

    - name: Cache dependencies
      uses: Swatinem/rust-cache@v2

    - name: Check formatting
      run: cargo fmt --all -- --check

    - name: Run clippy
      run: cargo clippy -- -D warnings

    - name: Run tests
      run: cargo test --all-targets

    - name: Run integration tests
      run: cargo test --test integration

    - name: Generate coverage
      if: matrix.os == 'ubuntu-latest' && matrix.rust == 'stable'
      uses: codecov/codecov-action@v3
```

### Optimizaciones de Performance
```yaml
# Cache inteligente para CI
- name: Cache cargo registry
  uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

- name: Cache target directory
  uses: actions/cache@v3
  with:
    path: target
    key: ${{ runner.os }}-target-${{ hashFiles('**/Cargo.lock') }}
```

## RELEASE AUTOMATION

### Workflow de Release
```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*.*.*'

jobs:
  release:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4

    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Build release
      run: cargo build --release

    - name: Run tests
      run: cargo test --release

    - name: Create GitHub release
      uses: softprops/action-gh-release@v1
      with:
        generate_release_notes: true
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

    - name: Publish to crates.io
      run: cargo publish --token ${{ secrets.CRATES_IO_TOKEN }}
```

### Versionado Automático
```yaml
# Configuración para versionado semántico
- name: Bump version
  uses: paulhatch/semantic-version@v5
  with:
    tag_prefix: "v"
    major_pattern: "(MAJOR)"
    minor_pattern: "(MINOR)"
    format: "${major}.${minor}.${patch}"
```

## SECURITY SCANNING

### Cargo Audit Integration
```yaml
# .github/workflows/security.yml
name: Security Scan

on:
  schedule:
    - cron: '0 0 * * 0'  # Weekly
  pull_request:
    paths:
      - '**/Cargo.lock'
      - '**/Cargo.toml'

jobs:
  audit:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4

    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Install cargo-audit
      run: cargo install cargo-audit

    - name: Run audit
      run: cargo audit

    - name: CodeQL Analysis
      uses: github/codeql-action/init@v2
      with:
        languages: rust

    - name: Perform CodeQL Analysis
      uses: github/codeql-action/analyze@v2
```

## DEPENDABOT CONFIGURATION

### Actualizaciones Automáticas
```yaml
# .github/dependabot.yml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 10
    reviewers:
      - "team/core"
    assignees:
      - "maintainer"
    commit-message:
      prefix: "deps"
      include: "scope"

  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
```

## ISSUE TEMPLATES

### Bug Report Template
```markdown
# .github/ISSUE_TEMPLATE/bug_report.md
---
name: Bug report
about: Create a report to help us improve
title: "[BUG] "
labels: bug
assignees: ''

---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Screenshots**
If applicable, add screenshots to help explain your problem.

**Environment:**
 - OS: [e.g. Ubuntu 20.04]
 - Rust version: [e.g. 1.70.0]
 - TRAE-CLI version: [e.g. 1.0.0]

**Additional context**
Add any other context about the problem here.
```

### Feature Request Template
```markdown
# .github/ISSUE_TEMPLATE/feature_request.md
---
name: Feature request
about: Suggest an idea for this project
title: "[FEATURE] "
labels: enhancement
assignees: ''

---

**Is your feature request related to a problem? Please describe.**
A clear and concise description of what the problem is. Ex. I'm always frustrated when [...]

**Describe the solution you'd like**
A clear and concise description of what you want to happen.

**Describe alternatives you've considered**
A clear and concise description of any alternative solutions or features you've considered.

**Additional context**
Add any other context or screenshots about the feature request here.
```

## PULL REQUEST TEMPLATE

### Template para PRs
```markdown
# .github/PULL_REQUEST_TEMPLATE.md
## Description
Please include a summary of the changes and the related issue. Please also include relevant motivation and context.

## Type of change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Refactoring (no functional changes)

## Checklist
- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review of my code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] Any dependent changes have been merged and published in downstream modules

## Testing
Describe the tests that you ran to verify your changes.

## Screenshots (if appropriate)
Add screenshots to help explain your changes.

## Additional Notes
Add any other context about the pull request here.
```

## CODEOWNERS

### Configuración de Code Owners
```plaintext
# .github/CODEOWNERS
# Global owners
* @project-owner @core-team

# Core functionality
src/core/ @core-team
src/commands/ @cli-team

# Documentation
docs/ @docs-team
*.md @docs-team

# CI/CD
.github/ @devops-team
```

## INTEGRACIÓN CON TRAE-CLI

### Comandos TRAE-CLI para GitHub
```bash
# Validar configuración de GitHub
trae github --validate

# Ejecutar workflows localmente
trae ci --simulate

# Generar templates
trae github --generate-templates

# Verificar dependencias
trae deps --check-updates
```

### Configuración TRAE-CLI
```toml
# Cargo.toml - Configuración GitHub
[package.metadata.trae.github]
workflows = ["ci", "release", "security"]
templates = ["bug_report", "feature_request", "pr"]
codeowners = true
dependabot = true
```

## TESTING DE WORKFLOWS

### Tests de CI Locales
```bash
# Ejecutar CI localmente con act
act -j test

# Ejecutar solo tests de Rust
act -j test --container-architecture linux/amd64

# Debug de workflows
act -j test --verbose
```

### Validación de Templates
```rust
// tests/github_templates_test.rs
#[test]
fn test_issue_templates() {
    // Verify templates exist and are valid
    assert!(Path::new(".github/ISSUE_TEMPLATE/bug_report.md").exists());
    assert!(Path::new(".github/ISSUE_TEMPLATE/feature_request.md").exists());

    // Check template content
    let bug_template = fs::read_to_string(".github/ISSUE_TEMPLATE/bug_report.md").unwrap();
    assert!(bug_template.contains("BUG"));
    assert!(bug_template.contains("Environment"));
}

#[test]
fn test_workflow_syntax() {
    // Validate YAML syntax of workflows
    let ci_workflow: serde_yaml::Value = serde_yaml::from_str(
        &fs::read_to_string(".github/workflows/ci.yml").unwrap()
    ).unwrap();

    assert!(ci_workflow.get("jobs").is_some());
    assert!(ci_workflow.get("on").is_some());
}
```

## MANEJO DE ERRORES

### Recuperación de Workflows
```bash
# Re-run failed workflow
gh workflow run ci.yml

# Ver logs detallados
gh run view <run-id> --log

# Debug local
act --list
act -j test --artifact-server-path /tmp/artifacts
```

### Troubleshooting
```yaml
# Workflow para debugging
name: Debug
on: workflow_dispatch

jobs:
  debug:
    runs-on: ubuntu-latest
    steps:
    - run: |
        echo "Debugging TRAE-CLI CI"
        rustc --version
        cargo --version
        echo "Environment variables:"
        env | grep -E "(RUST|CARGO)" | sort
```

## CONTEXTO DE DESARROLLO

La carpeta .github automatiza todo el ciclo de desarrollo de TRAE-CLI, desde CI/CD hasta gestión de releases. Garantiza calidad de código, seguridad y distribución automática.

### Puntos de Contacto
- **CI/CD Pipeline**: Workflows ejecutan automáticamente en pushes/PRs
- **Release Management**: Tags generan releases automáticamente
- **Security**: Escaneos semanales y en dependencias
- **TRAE-CLI**: Comandos integrados para gestión de GitHub

### Mejores Prácticas
- Mantener workflows modulares y reutilizables
- Usar caching para optimizar tiempos de CI
- Configurar dependabot para actualizaciones seguras
- Documentar procesos de release claramente
- Monitorear costos de GitHub Actions
- Usar secrets para tokens sensibles
- Implementar branch protection rules
- Configurar required status checks
