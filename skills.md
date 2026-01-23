# GitHub Copilot Agent Skills

## Descripción General

**Agent Skills** son una nueva capacidad introducida en GitHub Copilot que permite enseñar al asistente cómo realizar tareas especializadas de manera repetible y consciente del contexto. Las skills son carpetas que contienen instrucciones, scripts, ejemplos y recursos que GitHub Copilot carga automáticamente cuando detecta que son relevantes para tu solicitud.

## Características Principales

### 1. Capacidades de Agent Skills

- **Automatización Especializada**: Permiten definir flujos de trabajo personalizados que Copilot puede ejecutar de forma autónoma
- **Consciente del Contexto**: Las skills se cargan automáticamente cuando son relevantes al prompt del usuario
- **Multiplataforma**: Funcionan en Copilot coding agent, Copilot CLI y Visual Studio Code
- **Portabilidad**: Se pueden compartir entre proyectos, usuarios y organizaciones

### 2. Ubicaciones de Skills

GitHub Copilot busca skills en las siguientes ubicaciones:

```
# Nivel de proyecto
.github/skills/

# Nivel de usuario
~/.copilot/skills/

# Nivel de organización (próximamente)
# Definidas en la configuración de la organización
```

### 3. Estructura de una Skill

Cada skill consiste en:

```
.github/skills/nombre-de-skill/
├── SKILL.md              # Archivo principal con instrucciones
├── ejemplos/             # Ejemplos de código (opcional)
├── scripts/              # Scripts de automatización (opcional)
└── recursos/             # Recursos adicionales (opcional)
```

## Formato de SKILL.md

Cada skill debe tener un archivo `SKILL.md` con el siguiente formato:

```markdown
---
name: "Nombre de la Skill"
description: "Descripción breve de qué hace la skill"
author: "Tu nombre o equipo"
version: "1.0.0"
triggers: ["palabra_clave1", "palabra_clave2"]
---

# Instrucciones de la Skill

## Propósito
Describe el propósito principal de esta skill.

## Cuándo Usar
Define en qué situaciones esta skill debe ser utilizada.

## Pasos
1. Paso uno: Describe qué hacer
2. Paso dos: Más detalles
3. Paso tres: Finalización

## Ejemplos

### Ejemplo 1
\```rust
// Código de ejemplo
\```

### Ejemplo 2
\```rust
// Otro ejemplo
\```

## Consideraciones Especiales
- Punto importante 1
- Punto importante 2
```

## Casos de Uso Comunes

### 1. Automatización de Tests

```markdown
---
name: "Test Automation"
description: "Genera tests unitarios siguiendo las convenciones del proyecto"
triggers: ["test", "prueba", "testing"]
---

# Automatización de Tests

## Pasos
1. Analizar el código fuente
2. Identificar funciones públicas
3. Generar tests con casos edge
4. Incluir assertions apropiados
```

### 2. Revisión de Código

```markdown
---
name: "Code Review"
description: "Realiza revisión de código según estándares del proyecto"
triggers: ["review", "revisar", "check"]
---

# Revisión de Código

## Criterios
- Sin warnings del compilador
- Manejo explícito de errores
- Documentación completa
- Tests incluidos
```

### 3. Formateo y Estilo

```markdown
---
name: "Code Style"
description: "Aplica estándares de código del proyecto"
triggers: ["format", "style", "lint"]
---

# Formateo de Código

## Estándares
- Usar `cargo fmt`
- Aplicar `cargo clippy`
- Seguir convenciones Rust
```

## Diferencias: Agent Skills vs Custom Instructions

| Característica | Agent Skills | Custom Instructions |
|---------------|--------------|---------------------|
| **Propósito** | Enseñar flujos de trabajo especializados | Definir estándares de código |
| **Alcance** | Específico a tareas, carga bajo demanda | Glob pattern, siempre activas |
| **Contenido** | Scripts, ejemplos, recursos | Solo instrucciones |
| **Portabilidad** | Todos los agentes compatibles | VS Code/GitHub solamente |
| **Formato** | Directorio con múltiples archivos | Archivo único de texto |

## Skills Comunitarias

GitHub mantiene una colección de skills comunitarias:

- **Repositorio**: [github/awesome-copilot](https://github.com/topics/awesome-copilot)
- **Skills Populares**: 
  - Migraciones de código
  - Generación de documentación
  - Optimización de performance
  - Análisis de seguridad

## Mejores Prácticas

### 1. Diseño de Skills

- **Específicas**: Cada skill debe tener un propósito único y claro
- **Reutilizables**: Diseñar para uso en múltiples proyectos
- **Documentadas**: Incluir ejemplos y casos de uso
- **Versionadas**: Usar control de versiones semántico

### 2. Triggers Efectivos

```yaml
# Buenos triggers - específicos y únicos
triggers: ["cargo-analyze", "rust-safety", "performance-check"]

# Evitar triggers genéricos
triggers: ["check", "run", "test"]  # Demasiado amplios
```

### 3. Ejemplos Claros

```rust
// ✅ BUENO: Ejemplo específico y completo
/// Analiza código en busca de unwrap()
pub fn check_unwrap_usage(code: &str) -> Vec<Issue> {
    // Implementación completa
}

// ❌ MALO: Ejemplo incompleto
pub fn check_code() {
    // TODO: implementar
}
```

## Integración con TRAE-CLI

Para integrar skills con TRAE-CLI:

```bash
# Crear directorio de skills
mkdir -p .github/skills/trae-analysis

# Crear skill personalizada
cat > .github/skills/trae-analysis/SKILL.md << 'EOF'
---
name: "TRAE Analysis"
description: "Análisis de código Rust con métricas Six Sigma"
triggers: ["trae", "analyze", "six-sigma"]
---

# TRAE Analysis Skill

## Comandos
- `cargo trae analyze` - Análisis completo
- `cargo trae repair` - Reparación automática
- `cargo trae clippy --strict` - Linting estricto
EOF
```

## Skills para Rust

### Ejemplo: Rust Safety Check

```markdown
---
name: "Rust Safety Check"
description: "Detecta código unsafe y patrones inseguros"
triggers: ["safety", "unsafe", "seguridad"]
---

# Rust Safety Check

## Detecciones
1. Bloques `unsafe`
2. Llamadas a `unwrap()` y `expect()`
3. Llamadas a `panic!()`
4. Uso de `transmute`

## Acción
Para cada detección:
1. Reportar ubicación exacta
2. Sugerir alternativa segura
3. Indicar nivel de riesgo
```

### Ejemplo: Rust Performance

```markdown
---
name: "Rust Performance"
description: "Optimización de código Rust"
triggers: ["performance", "optimizar", "speed"]
---

# Rust Performance Optimization

## Áreas de Análisis
1. Clonaciones innecesarias
2. Allocaciones en loops
3. Uso de `.collect()` múltiple
4. Paralelización con rayon

## Métricas
- Tiempo de compilación
- Tamaño del binario
- Uso de memoria
```

## Migración desde Claude Skills

Si tienes skills de Claude Code, puedes migrarlas:

```bash
# Estructura Claude
claude_skills/
└── my-skill.md

# Convertir a GitHub Copilot
mkdir -p .github/skills/my-skill
mv claude_skills/my-skill.md .github/skills/my-skill/SKILL.md

# Agregar frontmatter YAML si no existe
```

## Recursos Adicionales

### Documentación Oficial

- [About Agent Skills - GitHub Docs](https://docs.github.com/en/copilot/concepts/agents/about-agent-skills)
- [Use Agent Skills in VS Code](https://code.visualstudio.com/docs/copilot/customization/agent-skills)
- [GitHub Copilot Tutorials](https://docs.github.com/en/copilot/tutorials)

### Comunidad y Ejemplos

- [GitHub Copilot Blog](https://github.blog/ai-and-ml/github-copilot/)
- [Microsoft Learn - Copilot Fundamentals](https://learn.microsoft.com/en-us/training/paths/copilot/)
- [Awesome Copilot Collection](https://github.com/topics/awesome-copilot)

## Actualizaciones Recientes

- **Diciembre 2025**: Lanzamiento oficial de Agent Skills
- **Enero 2026**: Soporte en VS Code Stable
- **Próximamente**: Skills a nivel de organización/empresa

## Conclusión

Agent Skills representan un avance significativo en la personalización de GitHub Copilot, permitiendo crear flujos de trabajo especializados y repetibles que se adaptan a las necesidades específicas de cada proyecto y equipo. Al combinar instrucciones, ejemplos y scripts, las skills proporcionan una forma poderosa de extender las capacidades de Copilot manteniendo consistencia y calidad en el desarrollo.
