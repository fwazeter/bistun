#!/bin/bash

# Target file
FILE="data/snapshot.json"
TMP_FILE="data/snapshot_tmp.json"

echo "Applying Phase B schema updates to $FILE..."

# Use jq to iterate over the .profiles array and append the new fields to every object
jq '.profiles |= map(. + {
  "unicode_blocks": ["Basic Latin"],
  "normalization": "NFC",
  "transliteration": "NONE"
})' "$FILE" > "$TMP_FILE"

# Replace the old file with the new one
mv "$TMP_FILE" "$FILE"

echo "✅ Successfully injected unicode_blocks, normalization, and transliteration into all profiles!"