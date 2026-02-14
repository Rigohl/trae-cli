# FUTURS / Roadmap items (saved)

Fecha: 2026-02-14

Resumen rápido
- Integrar GitHub Copilot (CLI + SDK) con TRAE-CLI para permitir workflows agénticos (resúmenes, generación de PRs, análisis automáticos).
- Persistir decisiones y tareas importantes en MCP Memory y sincronizarlas con Notion (workspace de documentación del proyecto).
- Añadir ejemplos, tests y comandos `trae copilot` (wrappers a Copilot CLI o SDK).

Tareas concretas
1. Integración Copilot CLI + SDK
   - Crear `trae copilot` subcomando que permita invocar `copilot` o `gh copilot` (summarize, create-pr, analyze).
   - Añadir ejemplo `examples/copilot-sdk-node` con `CopilotClient` que hace: session.createSession -> sendAndWait.
   - Tests de integración (mocked SDK) y smoke test en CI.

2. MCP Memory (persistente)
   - Añadir API/CLI para escribir/leer entradas de memoria (`trae memory put/get/list`).
   - Almacenar "decisiones de diseño" y "FUTURS" en MCP Memory (local `.trae/mcp_memory.json`) y exponer endpoints para lectura.
   - Opcional: desplegar `memory_p` MCP server en CI para pruebas end-to-end.

3. Notion sync (documentación)
   - Crear exportador/importador Notion (opcional) que publique páginas con la estructura de FUTURS.
   - Añadir documentación y script `scripts/push-to-notion.ps1` que usa `NOTION_TOKEN`/integration.

4. UX y QA
   - Añadir Script Commands para Raycast / VS Code tasks para invocar Copilot workflows desde el editor.
   - Documentar y asegurar permisos (trusted directories, approval options).

Criterios de aceptación
- `trae copilot` funciona localmente para al menos 3 workflows (summary, create-pr, analyze).
- FUTURS aparece en MCP Memory (local `.trae/mcp_memory.json`) y en `docs/notion/COPILOT-INTEGRATION.md`.
- PR con tests y ejemplos creado y aprobado.

Próximos pasos sugeridos
- (T1) Implementar `trae copilot` wrapper — prioridad alta.
- (T2) Añadir `trae memory` CRUD para MCP Memory — prioridad media.
- (T3) Crear Notion-sync script y documentar permisos — prioridad baja.

---

(Entrada guardada localmente en `.trae/mcp_memory.json` y en `docs/notion/COPILOT-INTEGRATION.md` como borrador.)