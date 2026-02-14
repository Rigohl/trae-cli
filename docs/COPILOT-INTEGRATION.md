# Copilot CLI & SDK integration (TRA E-CLI)

Estado: borrador (guardado localmente / preparado para Notion)

Objetivo
- Integrar GitHub Copilot (CLI + SDK) con TRAE-CLI para permitir flujos agénticos: resúmenes, generación de PRs, análisis y persistencia de decisiones en MCP Memory y Notion.

Resumen técnico
- Opciones de integración:
  1. `trae copilot` shell wrapper que invoca `copilot` o `gh copilot`.
  2. `examples/copilot-sdk-node` que usa `@github/copilot-sdk` para sesiones programáticas y herramientas personalizadas.
  3. `trae memory` (CLI) para CRUD sobre MCP Memory (persistencia local en `.trae/mcp_memory.json` o via MCP server).

Requisitos
- Copilot CLI instalado y autenticado (`copilot login`) o `gh copilot` disponible.
- Para CI/automation usar `COPILOT_GITHUB_TOKEN` / `GH_TOKEN` / `GITHUB_TOKEN`.
- Para Notion sync: `NOTION_TOKEN` (integration) con permisos para crear/editar páginas.

Ejemplo rápido (wrapper `trae copilot`)
- `trae copilot summary README.md` → ejecuta `copilot -p "Resume README.md" --model gemini-3-pro-preview` y formatea salida.

Ejemplo Node (SDK minimal)
```ts
import { CopilotClient } from "@github/copilot-sdk";
const client = new CopilotClient();
const session = await client.createSession({ model: "gemini-3-pro-preview" });
const r = await session.sendAndWait({ prompt: "Resume el README del repo" });
console.log(r?.data?.content);
await client.stop();
```

MCP Memory (diseño)
- Implementar `trae memory put "key" "value"` y `trae memory get "key"`.
- Local fallback: `.trae/mcp_memory.json` (JSON array).
- MCP server mode: `copilot --headless --port 4321` o `memory_p` si existe; SDK conectar a `cliUrl`.

Notion sync (diseño)
- Script `scripts/push-to-notion.ps1` que convierte entradas de FUTURS.md a página Notion.
- Usar `NOTION_TOKEN` y target database/page id en env var.

Seguridad y permisos
- Usar trusted directories para Copilot CLI y `--allow-tool` de forma explícita.
- En CI, usar env vars en lugar de interactive login.

Siguientes pasos
1. Añadir `trae copilot` subcomando (wrapper + tests).
2. Añadir `trae memory` CRUD (local + optional MCP server).
3. Crear `examples/copilot-sdk-node` y PR de ejemplo.
4. Añadir Notion sync script y documentación.
