#!/bin/bash

DOMAIN="https://konnektoren.help"

CURRENT_DATE=$(date +%Y-%m-%d)

PAGES=("/" "/about" "/konnektoren" "/verbs" "/.well-known/did.json")

echo '<?xml version="1.0" encoding="UTF-8"?>' > sitemap.xml
echo '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">' >> sitemap.xml

for PAGE in "${PAGES[@]}"; do
  echo "  <url>" >> sitemap.xml
  echo "    <loc>${DOMAIN}${PAGE}</loc>" >> sitemap.xml
  echo "    <lastmod>${CURRENT_DATE}</lastmod>" >> sitemap.xml
  echo "  </url>" >> sitemap.xml
done

echo '</urlset>' >> sitemap.xml
