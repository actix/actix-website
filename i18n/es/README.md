# Spanish Translations

Spanish uses the `es` locale and the `/es/` URL prefix.
Use general Spanish that readers in different regions can understand.
Follow the [shared translation guide](../README.md) for common rules and the update workflow.

## Scope

Spanish currently covers the Welcome page and the homepage navigation and hero section.
The rest of the homepage and all other pages use English content.

- Translate `docs/welcome.md` in `docusaurus-plugin-content-docs/current/welcome.md`.
- Translate homepage hero messages in `code.json`.
- Translate navigation and sidebar labels in the plugin and theme JSON files.
- Translate configuration messages from `src/data/site-messages.json` in `site.json`.

## Terminology

- **Actix Web:** Keep the project name unchanged.
- **handler:** manejador.
- **extractor:** extractor.
- **request:** solicitud.
- **response:** respuesta.
- **application state:** estado de la aplicación.
- **scope:** ámbito.
- **guard:** guarda.
- **worker thread:** hilo de trabajo.
- **graceful shutdown:** apagado ordenado.
- **middleware, crate, and runtime API identifiers:** Keep official spellings. Explain them in Spanish when needed.

## Validation Commands

Run these commands from the repository root at the corresponding steps in the shared update workflow:

- **Record source revisions:** `pnpm run i18n:record`. This updates `translation-sources.json` with the current English source hashes after translation review. It does not translate or review text.
- **Check source translations:** `pnpm run i18n:check`.
- **Check the production build:** `pnpm run i18n:check-build`.

## Search and Routing

Local search uses `docusaurus-lunr-search` and keeps its English interface.
The Spanish index uses Spanish and English tokenization so API names remain searchable.
URLs use trailing slashes so homepage language links resolve correctly.
The language-link tests cover homepages and nested documentation pages with and without trailing slashes.
The build check verifies route coverage, language metadata, internal links, heading anchors,
localized edit links, and search isolation. Netlify uses `static/_redirects` for Spanish 404 responses.
