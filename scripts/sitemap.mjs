/**
 * Check for an exact URL match in a sitemap `<loc>` element.
 *
 * URLs must match as written, including trailing slashes and query strings.
 *
 * @param {string} sitemap Sitemap XML text.
 * @param {string} expectedUrl Complete URL to find.
 * @returns {boolean} Whether a `<loc>` value matches the URL.
 */
export function hasSitemapUrl(sitemap, expectedUrl) {
  return [...sitemap.matchAll(/<loc>([^<]+)<\/loc>/g)].some(
    ([, location]) => location === expectedUrl,
  );
}
