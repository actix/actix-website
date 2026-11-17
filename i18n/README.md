# Translations

English is the source language. Spanish uses the `es` locale and the `/es/` URL prefix.
Use general Spanish that readers in different regions can understand.

Spanish currently covers the Welcome page and the homepage navigation and hero section.
The rest of the homepage and all other pages use English content.
Docusaurus loads the English source when a translated Markdown file is absent.

## Content Locations

- Translate `docs/welcome.md` in `es/docusaurus-plugin-content-docs/current/welcome.md`.
- Translate homepage hero messages in `es/code.json`. Keep stable message IDs in `<Translate>` and `translate()` calls.
- Translate navigation and sidebar labels in the plugin JSON files.
- Translate configuration messages from `src/data/site-messages.json` in `es/site.json`.

Keep document IDs, filenames, slugs, explicit heading IDs, code identifiers, and official names unchanged.
Import Rust examples and version values from the shared sources.
Use Spanish links for pages in other sections, such as `/es/community/crates`.
Use relative Markdown links within the same section.

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

## Update a Translation

1. Update the Spanish page when the English source changes. Check technical meaning against the source.
2. Run `pnpm run write-translations --locale es` after adding React messages or navigation labels. Translate new messages and keep their placeholders, such as `{command}`. Remove generated catalog files for sections that remain in English.
3. Preview Spanish with `just preview es`. The development server runs one locale at a time. Test search against a production build.
4. Run `pnpm run i18n:record` after reviewing the affected translations. This updates `es/translation-sources.json` with the current English source hashes. It does not translate or review text.
5. Run `pnpm run test:i18n`, `pnpm run i18n:check`, `pnpm run build`, and `pnpm run i18n:check-build`.

The coverage check tracks existing translations and their entries in `es/translation-sources.json`.
It reports missing maintained translations, missing or untranslated messages, changed placeholders,
changed routes, and English source changes that need review. Untranslated pages and catalogs
do not need copies. To stop maintaining a translation, remove its file and its source entries from
`es/translation-sources.json`. To add a translation, copy and translate its source, then record its revision.

The check recreates `i18n/en/` for extraction;
that generated directory is ignored by Git. A source hash records a review baseline, not proof of translation quality.

Both locales use local search. Spanish includes Spanish and English tokenization so API names remain searchable.
The search dependency patch makes both search fields use the translated accessibility label.
The Docusaurus theme patch keeps homepage language links valid when `/es/` becomes `/es`.
The language-link tests cover both homepage forms and a nested documentation page.
The build check verifies route coverage, language metadata, internal links, heading anchors,
localized edit links, and search isolation. Netlify uses `static/_redirects` for Spanish 404 responses.
