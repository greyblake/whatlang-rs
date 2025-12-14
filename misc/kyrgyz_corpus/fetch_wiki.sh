#!/bin/bash
# Fetch Kyrgyz Wikipedia articles for trigram generation

ARTICLES=(
    "Кыргызстан"
    "Кыргыз_тили"
    "Бишкек"
    "Ош"
    "Тарых"
    "Кыргыз_элинин_тарыхы"
    "Манас"
    "Ысык-Көл"
    "Ала-Тоо"
    "Кыргыз_маданияты"
    "Түрк_тилдери"
    "Ислам"
    "Кыргыз_Республикасынын_Президенти"
    "Музыка"
    "Адабият"
    "Саясат"
    "Экономика"
    "Билим_берүү"
    "Табият"
    "Спорт"
)

OUTPUT_FILE="kyrgyz_corpus.txt"
> "$OUTPUT_FILE"

for article in "${ARTICLES[@]}"; do
    encoded=$(python3 -c "import urllib.parse; print(urllib.parse.quote('$article'))")
    url="https://ky.wikipedia.org/w/api.php?action=query&titles=$encoded&prop=extracts&explaintext=true&format=json"
    
    content=$(curl -s -L -A "Mozilla/5.0 (compatible; WhatlangBot/1.0)" "$url" | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    pages = data.get('query', {}).get('pages', {})
    for page_id, page in pages.items():
        if 'extract' in page:
            print(page['extract'])
except:
    pass
")
    if [ -n "$content" ]; then
        echo "$content" >> "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
    fi
done

echo "Corpus size: $(wc -c < "$OUTPUT_FILE") bytes"
