# Translations

English is the source language. Each translation has its own `i18n/<locale>/` directory.
Read that directory's `README.md` for its scope, terminology, and validation commands.

## File Structure

```text
i18n/
  README.md
  <locale>/
    README.md
    code.json
    site.json
    docusaurus-plugin-content-docs/
      current.json
      current/
        <document>.md
    docusaurus-theme-classic/
      navbar.json
```

Keep all guidance, catalogs, and translated content for a language in its locale directory.
When adding a language, add its guide there. This shared guide does not list individual languages.

`code.json` contains React messages. Plugin and theme JSON files contain sidebar and navigation labels.
`site.json` translates configuration messages from `src/data/site-messages.json`.
Translated Markdown files follow the source directory structure within the corresponding content plugin directory.
Add only files for maintained translations. Docusaurus loads the English source when a translated Markdown file is absent.

## Shared Rules

Keep document IDs, filenames, slugs, explicit heading IDs, code identifiers, and official names unchanged.
Keep stable message IDs in `<Translate>` and `translate()` calls, and preserve placeholders such as `{command}`.
Import Rust examples and version values from the shared sources.
Use trailing slashes in page URLs and locale-prefixed links for pages in other sections, such as `/<locale>/community/crates/`.
Use relative Markdown links within the same section.

## Update a Translation

Replace `<locale>` in commands with the locale directory name.

1. Read the locale's guide and update the translation when its English source changes. Check technical meaning against the source.
2. Run `pnpm run write-translations --locale <locale>` after adding React messages or navigation labels. Translate new messages. Remove generated catalog files for sections that remain in English.
3. Preview with `just preview <locale>`. The development server runs one locale at a time. Test search against a production build.
4. Run `pnpm run test:i18n`, the locale's translation check, `pnpm run build`, and the locale's build check.

The translation check validates existing translated files and message catalogs.
It reports empty translations, missing or obsolete message keys, changed placeholders,
changed route fields, and translated files whose source no longer exists.
Pages and catalogs without translations use English content and do not need copies.
To add a translation, copy and translate its source. To stop maintaining one, remove its file.

The check recreates `i18n/en/` for extraction;
that generated directory is ignored by Git.
