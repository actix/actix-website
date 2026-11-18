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

export function checkTranslations(root, { locale = "es" } = {}) {
  const errors = [];
  const localeDirectory = `i18n/${locale}`;

  for (const [sourceDirectory, targetDirectory] of contentDirectories) {
    const extension = sourceDirectory === "static/img/diagrams" ? /\.mmd$/ : /\.mdx?$/;

    for (const file of filesIn(path.join(root, localeDirectory, targetDirectory)).filter((file) =>
      extension.test(file),
    )) {
      const sourceFile = `${sourceDirectory}/${file}`;
      const targetFile = `${localeDirectory}/${targetDirectory}/${file}`;

      if (!fs.existsSync(path.join(root, sourceFile))) {
        errors.push(`Remove obsolete translation: ${targetFile}`);
        continue;
      }

      const source = fs.readFileSync(path.join(root, sourceFile), "utf8");
      const translated = fs.readFileSync(path.join(root, targetFile), "utf8");

      if (!translated.trim()) {
        errors.push(`Empty translation: ${targetFile}`);
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
    if (!fs.existsSync(path.join(root, targetFile))) continue;

    const original = JSON.parse(fs.readFileSync(path.join(root, sourceFile), "utf8"));
    const translated = JSON.parse(fs.readFileSync(path.join(root, targetFile), "utf8"));

    for (const [key, entry] of Object.entries(original)) {
      const message = plainMessages ? entry : entry.message;
      const translation = plainMessages ? translated[key] : translated[key]?.message;

      if (typeof translation !== "string" || !translation.trim()) {
        errors.push(`Missing message: ${targetFile}#${key}`);
        continue;
      }

      if (JSON.stringify(placeholders(message)) !== JSON.stringify(placeholders(translation))) {
        errors.push(`Placeholder mismatch: ${targetFile}#${key}`);
      }
    }

    for (const key of Object.keys(translated)) {
      if (!(key in original)) errors.push(`Removed source message: ${targetFile}#${key}`);
    }
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

  const errors = checkTranslations(process.cwd());

  if (errors.length > 0) {
    console.error(errors.join("\n"));
    process.exitCode = 1;
  } else {
    console.log("Spanish translation structure is valid.");
  }
}
