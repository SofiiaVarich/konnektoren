#!/bin/bash

DOMAIN="https://konnektoren.help"

CURRENT_DATE=$(date +%Y-%m-%d)

PAGES=("/" "/about" "/adjectives" "/konnektoren" "/verbs" "/leaderboard")
LANGS=("ar" "cn" "de" "en" "es" "pl" "tr" "ua")

echo '<?xml version="1.0" encoding="UTF-8"?>' > sitemap.xml
echo '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">' >> sitemap.xml

for PAGE in "${PAGES[@]}"; do
  echo "  <url>" >> sitemap.xml
  echo "    <loc>${DOMAIN}${PAGE}</loc>" >> sitemap.xml
  echo "    <lastmod>${CURRENT_DATE}</lastmod>" >> sitemap.xml

  echo "    <xhtml:link" >> sitemap.xml
  echo "      rel=\"alternate\"" >> sitemap.xml
  echo "      hreflang=\"en\"" >> sitemap.xml
  echo "      href=\"${DOMAIN}${PAGE}\"/>" >> sitemap.xml

    for LANG in "${LANGS[@]}"; do
        echo "    <xhtml:link" >> sitemap.xml
        echo "      rel=\"alternate\"" >> sitemap.xml
        echo "      hreflang=\"${LANG}\"" >> sitemap.xml
        echo "      href=\"${DOMAIN}${PAGE}?lang=${LANG}\"/>" >> sitemap.xml
    done

    echo "  </url>" >> sitemap.xml
done

echo '</urlset>' >> sitemap.xml
