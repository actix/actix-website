import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
import { hasSitemapUrl } from "./sitemap.mjs";

const require = createRequire(import.meta.url);
const lunr = require("lunr");

require("lunr-languages/lunr.stemmer.support")(lunr);
require("lunr-languages/lunr.es")(lunr);
require("lunr-languages/lunr.multi")(lunr);

const buildDirectory = path.resolve("build");
const pages = new Map();

for (const file of fs
  .readdirSync(buildDirectory, { recursive: true })
  .filter((file) => file.endsWith(".html"))) {
  pages.set(file, fs.readFileSync(path.join(buildDirectory, file), "utf8"));
}

function pageFor(url) {
  const route = decodeURIComponent(url.pathname).replace(/^\//, "").replace(/\/$/, "");

  return [`${route}.html`, `${route}/index.html`, route === "" ? "index.html" : route].find(
    (candidate) => pages.has(candidate),
  );
}

let spanishPages = 0;

for (const [file, html] of pages) {
  if (file.startsWith("es/") || fs.existsSync(path.join("static", file))) continue;

  const translatedFile = `es/${file}`;
  const translated = pages.get(translatedFile);

  assert.ok(translated, `Missing Spanish route for ${file}`);
  assert.match(html, /<html[^>]+lang="en"/, file);
  assert.match(translated, /<html[^>]+lang="es"/, translatedFile);

  if (file !== "404.html") {
    assert.match(translated, /rel="alternate"[^>]+hreflang="en"/, translatedFile);
    assert.match(translated, /rel="alternate"[^>]+hreflang="es"/, translatedFile);
    assert.match(
      translated,
      /rel="canonical"[^>]+href="https:\/\/actix\.rs\/es\/(?:[^"]*\/)?"/,
      translatedFile,
    );
  }

  const route = `/es/${file.replace(/index\.html$/, "")}`;

  for (const [tag, href] of [...translated.matchAll(/<a\b[^>]*\bhref="([^"]*)"[^>]*>/g)].map(
    (match) => [match[0], match[1]],
  )) {
    const url = new URL(href.replaceAll("&amp;", "&"), `https://actix.rs${route}`);

    if (url.origin !== "https://actix.rs") continue;

    const target = pageFor(url);

    if (!target) {
      assert.ok(
        fs.existsSync(path.join(buildDirectory, decodeURIComponent(url.pathname))),
        `Broken link in ${translatedFile}: ${href}`,
      );
      continue;
    }

    if (file === "index.html" || file === "docs/index.html") {
      assert.ok(
        url.pathname === "/es" || url.pathname.startsWith("/es/") || tag.includes('lang="en"'),
        `Link leaves Spanish in ${translatedFile}: ${href}`,
      );
    }

    if (url.hash) {
      const id = decodeURIComponent(url.hash.slice(1));

      assert.ok(
        pages.get(target).includes(`id="${id}"`),
        `Broken anchor in ${translatedFile}: ${href}`,
      );
    }
  }

  spanishPages += 1;
}

for (const [prefix, term, expectedPage] of [
  ["", "requests", "/docs/extractors/"],
  ["/es", "bienvenido", "/es/docs/"],
]) {
  const indexes = JSON.parse(
    fs.readFileSync(path.join(buildDirectory, prefix, "search-index.json"), "utf8"),
  );
  const documents = indexes.flatMap((index) => index.documents);

  assert.ok(documents.length > 0, `Empty search index: ${prefix}`);
  assert.ok(
    documents.every((doc) => (prefix ? doc.u.startsWith(`${prefix}/`) : !doc.u.startsWith("/es/"))),
    `Mixed search locales: ${prefix}`,
  );
  assert.ok(
    documents.every((doc) => !doc.u.includes("/docs/actix")),
    `Archived content is indexed: ${prefix}`,
  );

  const matches = indexes.flatMap((data) => {
    const index = lunr.Index.load(data.index);
    const results = index.search(term);

    return results.flatMap((result) =>
      data.documents.filter((doc) => String(doc.i) === result.ref),
    );
  });

  assert.ok(
    matches.some((doc) => doc.u === expectedPage),
    `Search cannot find ${expectedPage} for ${term}`,
  );
}

const spanishHome = pages.get("es/index.html");
const spanishWelcome = pages.get("es/docs/index.html");
const spanishSitemap = fs.readFileSync(path.join(buildDirectory, "es/sitemap.xml"), "utf8");

assert.ok(hasSitemapUrl(spanishSitemap, "https://actix.rs/es/docs/"));
assert.ok(spanishHome.includes("Actix Web es un framework web"));
assert.ok(spanishHome.includes("Primeros Pasos"));
assert.ok(spanishHome.includes("Pausar Animación"));
assert.ok(spanishHome.includes("Strong Type Safety"));
assert.ok(spanishHome.includes("Powerful Extractors"));
assert.ok(spanishHome.includes("Copy command:"));
assert.ok(!spanishHome.includes("Extractores Potentes"));
assert.ok(spanishWelcome.includes("Bienvenido a Actix Web"));
assert.ok(pages.get("es/docs/whatis/index.html").includes("What Is Actix Web"));
assert.ok(pages.get("es/docs/getting-started/index.html").includes("Installing Rust"));
assert.ok(
  pages.get("es/docs/extractors/index.html").includes("Type-Safe Request Information Extraction"),
);
assert.ok(
  spanishWelcome.includes("/edit/main/i18n/es/docusaurus-plugin-content-docs/current/welcome.md"),
);
assert.ok(
  pages.get("es/docs/getting-started/index.html").includes("/edit/main/docs/getting-started.md"),
);
assert.ok(
  pages
    .get(pageFor(new URL("https://actix.rs/es/community")))
    .includes("/edit/main/community/index.md"),
);

console.log(
  `Checked ${spanishPages} locale routes, English fallback, internal links, anchors, metadata, edit links, and both search indexes.`,
);
