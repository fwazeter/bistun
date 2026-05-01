#!/usr/bin/env bash
# Bistun LMS - Raw Data Ingestion Scraper
# Ref: [005-LMS-INGEST]

set -e
echo "🚀 Starting Bistun LMS Data Scraper..."

RAW_DIR="data/raw"
mkdir -p "$RAW_DIR"

# 1. ISO 639-3 (Language Typology)
#echo "⬇️  Downloading ISO 639-3 definitions..."
#curl -sL "https://iso639-3.sil.org/sites/iso639-3/files/downloads/iso-639-3.tab" -o "$RAW_DIR/iso-639-3.tab"

# 2. ISO 15924 (Script Orthography)
#echo "⬇️  Downloading ISO 15924 definitions..."
#curl -sL "https://www.unicode.org/iso15924/iso15924.txt" -o "$RAW_DIR/iso15924.txt"

# 3. Unicode CLDR (Orthography & Taxonomy)
CLDR_BASE="https://raw.githubusercontent.com/unicode-org/cldr-json/main/cldr-json/cldr-core"
echo "⬇️  Downloading Unicode CLDR Script Metadata..."
curl -sL "$CLDR_BASE/scriptMetadata.json" -o "$RAW_DIR/scriptMetadata.json"

echo "⬇️  Downloading Unicode CLDR Numbering Systems..."
curl -sL "$CLDR_BASE/supplemental/numberingSystems.json" -o "$RAW_DIR/numberingSystems.json"

echo "⬇️  Downloading Unicode CLDR Likely Subtags..."
curl -sL "$CLDR_BASE/supplemental/likelySubtags.json" -o "$RAW_DIR/likelySubtags.json"

echo "⬇️  Downloading Unicode CLDR Plural Rules..."
curl -sL "$CLDR_BASE/supplemental/plurals.json" -o "$RAW_DIR/plurals.json"

echo "✅ All raw standards downloaded successfully to $RAW_DIR/"