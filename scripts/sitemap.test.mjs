import assert from "node:assert/strict";
import { test } from "node:test";
import { hasSitemapUrl } from "./sitemap.mjs";

const expectedUrl = "https://actix.rs/es/docs";

test("finds the exact URL in a sitemap location", () => {
  const sitemap = `<urlset><url><loc>https://actix.rs/es/code</loc></url><url><loc>${expectedUrl}</loc></url></urlset>`;

  assert.equal(hasSitemapUrl(sitemap, expectedUrl), true);
});

for (const url of [
  "https://example.com/https://actix.rs/es/docs",
  "https://example.com/?next=https://actix.rs/es/docs",
  "https://actix.rs/es/docs-other",
  "https://actix.rs/es/docs?next=https://example.com",
]) {
  test(`rejects a URL substring in ${url}`, () => {
    const sitemap = `<urlset><url><loc>${url}</loc></url></urlset>`;

    assert.equal(hasSitemapUrl(sitemap, expectedUrl), false);
  });
}

test("ignores URLs outside sitemap location elements", () => {
  const sitemap = `<urlset><!-- ${expectedUrl} --><url><loc>https://example.com</loc></url></urlset>`;

  assert.equal(hasSitemapUrl(sitemap, expectedUrl), false);
});
