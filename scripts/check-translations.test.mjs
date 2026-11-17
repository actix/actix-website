import assert from "node:assert/strict";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { test } from "node:test";
import { checkTranslations } from "./check-translations.mjs";

function fixture(t) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "actix-translations-"));

  t.after(() => fs.rmSync(root, { recursive: true, force: true }));

  const write = (file, content) => {
    const destination = path.join(root, file);

    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, content);
  };

  write("docs/welcome.md", "---\nslug: /\n---\n\n# Welcome\n");
  write(
    "i18n/es/docusaurus-plugin-content-docs/current/welcome.md",
    "---\nslug: /\n---\n\n# Bienvenida\n",
  );
  write("i18n/en/code.json", JSON.stringify({ greeting: { message: "Hello, {name}" } }));
  write("i18n/es/code.json", JSON.stringify({ greeting: { message: "Hola, {name}" } }));
  write("src/data/site-messages.json", JSON.stringify({ tagline: "A web framework" }));
  write("i18n/es/site.json", JSON.stringify({ tagline: "Un framework web" }));

  assert.deepEqual(checkTranslations(root, { update: true }), []);

  return { root, write };
}

test("complete translations pass without changes to their revision record", (t) => {
  const { root } = fixture(t);

  assert.deepEqual(checkTranslations(root), []);
});

test("source pages without translations use English fallback", (t) => {
  const { root, write } = fixture(t);

  write("community/new.md", "# New Page\n");

  assert.deepEqual(checkTranslations(root), []);
});

test("catalogs outside the translated scope use English fallback", (t) => {
  const { root, write } = fixture(t);

  write(
    "i18n/en/docusaurus-plugin-content-docs-community/current.json",
    JSON.stringify({ label: { message: "Community" } }),
  );

  assert.deepEqual(checkTranslations(root), []);
});

test("an existing translation cannot be removed without updating its scope", (t) => {
  const { root } = fixture(t);

  fs.unlinkSync(path.join(root, "i18n/es/docusaurus-plugin-content-docs/current/welcome.md"));

  assert.ok(
    checkTranslations(root, { update: true }).includes(
      "Missing translation: i18n/es/docusaurus-plugin-content-docs/current/welcome.md",
    ),
  );
});

test("new translations require a source review before entering coverage", (t) => {
  const { root, write } = fixture(t);

  write("docs/new.md", "# New Page\n");
  write("i18n/es/docusaurus-plugin-content-docs/current/new.md", "# Página Nueva\n");

  assert.ok(checkTranslations(root).includes("Review changed source: docs/new.md"));
  assert.deepEqual(checkTranslations(root, { update: true }), []);
  assert.deepEqual(checkTranslations(root), []);
});

test("changed source text requires a translation review", (t) => {
  const { root, write } = fixture(t);

  write("docs/welcome.md", "---\nslug: /\n---\n\n# Welcome\n\nNew setup instructions.\n");

  assert.ok(checkTranslations(root).includes("Review changed source: docs/welcome.md"));
});

test("a deleted source page requires removal of its translation", (t) => {
  const { root } = fixture(t);

  fs.unlinkSync(path.join(root, "docs/welcome.md"));

  assert.ok(
    checkTranslations(root, { update: true }).some((error) =>
      error.startsWith("Remove obsolete translation:"),
    ),
  );
});

test("missing messages and lost placeholders cannot be marked as reviewed", (t) => {
  const { root, write } = fixture(t);
  const manifest = fs.readFileSync(path.join(root, "i18n/es/translation-sources.json"), "utf8");

  write(
    "i18n/en/code.json",
    JSON.stringify({ greeting: { message: "Hello, {name}" }, next: { message: "Next page" } }),
  );
  write("i18n/es/code.json", JSON.stringify({ greeting: { message: "Hola" } }));

  const errors = checkTranslations(root, { update: true });

  assert.ok(errors.includes("Placeholder mismatch: i18n/es/code.json#greeting"));
  assert.ok(errors.includes("Missing message: i18n/es/code.json#next"));
  assert.equal(
    fs.readFileSync(path.join(root, "i18n/es/translation-sources.json"), "utf8"),
    manifest,
  );
});

test("changed routes and untranslated messages fail coverage checks", (t) => {
  const { root, write } = fixture(t);

  write(
    "i18n/es/docusaurus-plugin-content-docs/current/welcome.md",
    "---\nslug: /bienvenida\n---\n\n# Bienvenida\n",
  );
  write("i18n/es/code.json", JSON.stringify({ greeting: { message: "Hello, {name}" } }));

  const errors = checkTranslations(root);

  assert.ok(errors.some((error) => error.startsWith("Keep the source slug:")));
  assert.ok(errors.includes("Untranslated message: i18n/es/code.json#greeting"));
});
