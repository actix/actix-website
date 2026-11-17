import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
import { test } from "node:test";
import { compileFunction } from "node:vm";

const themeRequire = createRequire(
  fs.realpathSync("node_modules/@docusaurus/theme-classic/package.json"),
);
const themeCommon = themeRequire.resolve("@docusaurus/theme-common");
const commonRequire = createRequire(themeCommon);
const { applyTrailingSlash } = commonRequire("@docusaurus/utils-common");
const source = fs
  .readFileSync(path.join(path.dirname(themeCommon), "utils/useAlternatePageUtils.js"), "utf8")
  .replace(/^import .*;$/gm, "")
  .replace("export function useAlternatePageUtils", "function useAlternatePageUtils");

// Test the installed hook with fixed context values. Its aliases exist only in the site bundle.
const loadHook = compileFunction(`${source}\nreturn useAlternatePageUtils();`, [
  "useDocusaurusContext",
  "useLocation",
  "applyTrailingSlash",
]);

for (const [pathname, english, spanish] of [
  ["/es/", "/", "/es/"],
  ["/es", "/", "/es/"],
  ["/es/docs/extractors", "/docs/extractors", "/es/docs/extractors"],
]) {
  test(`language links preserve the page at ${pathname}`, () => {
    const hook = loadHook(
      () => ({
        siteConfig: { baseUrl: "/es/", trailingSlash: false },
        i18n: {
          localeConfigs: {
            en: { url: "https://actix.rs", baseUrl: "/" },
            es: { url: "https://actix.rs", baseUrl: "/es/" },
          },
        },
      }),
      () => ({ pathname }),
      applyTrailingSlash,
    );

    assert.equal(hook.createUrl({ locale: "en", fullyQualified: false }), english);
    assert.equal(hook.createUrl({ locale: "es", fullyQualified: false }), spanish);
    assert.equal(
      hook.createUrl({ locale: "es", fullyQualified: true }),
      `https://actix.rs${spanish}`,
    );
  });
}
