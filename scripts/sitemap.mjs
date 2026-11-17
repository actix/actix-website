export function hasSitemapUrl(sitemap, expectedUrl) {
  return [...sitemap.matchAll(/<loc>([^<]+)<\/loc>/g)].some(
    ([, location]) => location === expectedUrl,
  );
}
