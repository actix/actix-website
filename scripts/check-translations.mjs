import { createHash } from "node:crypto";
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const contentDirectories = [
  ["docs", "docusaurus-plugin-content-docs/current"],
  ["community", "docusaurus-plugin-content-docs-community/current"],
  ["src/pages", "docusaurus-plugin-content-pages"],
  ["static/img/diagrams", "diagrams"],
];

const unchangedNames = new Set([
  "Actix Web",
  "actix",
  "actix-web",
  "actix-cors",
  "Actor",
  "Arbiter",
  "SyncArbiter",
  "Multipart",
]);

function filesIn(directory) {
  if (!fs.existsSync(directory)) return [];

  return fs
    .readdirSync(directory, { recursive: true })
    .filter((file) => fs.statSync(path.join(directory, file)).isFile())
    .sort();
}

function placeholders(message) {
  return [...new Set([...message.matchAll(/\{(\w+)\}/g)].map((match) => match[1]))].sort();
}

function sourceHash(content) {
  return createHash("sha256").update(content.replaceAll("\r\n", "\n")).digest("hex");
}

export function checkTranslations(root, { locale = "es", update = false } = {}) {
  const errors = [];
  const sources = {};
  const localeDirectory = `i18n/${locale}`;
  const manifestFile = path.join(root, localeDirectory, "translation-sources.json");
  const previous = fs.existsSync(manifestFile)
    ? JSON.parse(fs.readFileSync(manifestFile, "utf8"))
    : {};

  function read(file) {
    const absolute = path.join(root, file);

    if (!fs.existsSync(absolute)) {
      errors.push(`Missing translation: ${file}`);
      return undefined;
    }

    return fs.readFileSync(absolute, "utf8");
  }

  for (const [sourceDirectory, targetDirectory] of contentDirectories) {
    const extension = sourceDirectory === "static/img/diagrams" ? /\.mmd$/ : /\.mdx?$/;

    for (const file of filesIn(path.join(root, localeDirectory, targetDirectory)).filter((file) =>
      extension.test(file),
    )) {
      if (!fs.existsSync(path.join(root, sourceDirectory, file))) {
        errors.push(`Remove obsolete translation: ${localeDirectory}/${targetDirectory}/${file}`);
      }
    }

    for (const file of filesIn(path.join(root, sourceDirectory)).filter((file) =>
      extension.test(file),
    )) {
      const sourceFile = `${sourceDirectory}/${file}`;
      const targetFile = `${localeDirectory}/${targetDirectory}/${file}`;

      if (!fs.existsSync(path.join(root, targetFile)) && !(sourceFile in previous)) continue;

      const source = fs.readFileSync(path.join(root, sourceFile), "utf8");
      const translated = read(targetFile);

      sources[sourceFile] = sourceHash(source);

      if (translated === undefined) continue;

      if (!translated.trim() || translated === source) {
        errors.push(`Untranslated content: ${targetFile}`);
      }

      for (const field of ["id", "slug", "unlisted"]) {
        const pattern = new RegExp(`^${field}:.*$`, "m");

        if (source.match(pattern)?.[0] !== translated.match(pattern)?.[0]) {
          errors.push(`Keep the source ${field}: ${targetFile}`);
        }
      }
    }
  }

  const englishDirectory = path.join(root, "i18n/en");

  if (!fs.existsSync(englishDirectory)) {
    errors.push("Run pnpm run write-translations --locale en first.");
  }

  const catalogues = filesIn(englishDirectory)
    .filter((file) => file.endsWith(".json"))
    .map((file) => [`i18n/en/${file}`, `${localeDirectory}/${file}`, false]);

  catalogues.push(["src/data/site-messages.json", `${localeDirectory}/site.json`, true]);

  for (const [sourceFile, targetFile, plainMessages] of catalogues) {
    const maintained = Object.keys(previous).some((source) => source.startsWith(`${sourceFile}#`));

    if (!fs.existsSync(path.join(root, targetFile)) && !maintained) continue;

    const original = JSON.parse(fs.readFileSync(path.join(root, sourceFile), "utf8"));
    const translatedText = read(targetFile);

    if (translatedText === undefined) continue;

    const translated = JSON.parse(translatedText);

    for (const [key, entry] of Object.entries(original)) {
      const message = plainMessages ? entry : entry.message;
      const translation = plainMessages ? translated[key] : translated[key]?.message;

      sources[`${sourceFile}#${key}`] = sourceHash(message);

      if (typeof translation !== "string" || !translation.trim()) {
        errors.push(`Missing message: ${targetFile}#${key}`);
        continue;
      }

      if (JSON.stringify(placeholders(message)) !== JSON.stringify(placeholders(translation))) {
        errors.push(`Placeholder mismatch: ${targetFile}#${key}`);
      }

      const prose = message.replace(/\{\w+\}/g, "");

      if (translation === message && /[a-z]/i.test(prose) && !unchangedNames.has(message)) {
        errors.push(`Untranslated message: ${targetFile}#${key}`);
      }
    }

    for (const key of Object.keys(translated)) {
      if (!(key in original)) errors.push(`Removed source message: ${targetFile}#${key}`);
    }
  }

  if (!update) {
    for (const [source, hash] of Object.entries(sources)) {
      if (previous[source] !== hash) errors.push(`Review changed source: ${source}`);
    }

    for (const source of Object.keys(previous)) {
      if (!(source in sources)) errors.push(`Remove deleted source: ${source}`);
    }
  }

  if (update && errors.length === 0) {
    fs.writeFileSync(manifestFile, `${JSON.stringify(sources, null, 2)}\n`);
  }

  return errors;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  // Docusaurus merges catalogues. Recreate the generated English catalogue to detect deleted keys.
  fs.rmSync(path.resolve("i18n/en"), { recursive: true, force: true });

  const extraction = spawnSync("pnpm", ["run", "write-translations", "--locale", "en"], {
    stdio: "inherit",
  });

  if (extraction.error) throw extraction.error;
  if (extraction.status !== 0) process.exit(extraction.status ?? 1);

  const errors = checkTranslations(process.cwd(), { update: process.argv.includes("--update") });

  if (errors.length > 0) {
    console.error(errors.join("\n"));
    process.exitCode = 1;
  } else {
    console.log("Maintained Spanish translations and source revisions are current.");
  }
}
