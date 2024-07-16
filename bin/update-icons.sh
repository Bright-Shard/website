#!/bin/bash

# Removes unneeded glyphs from the Material Icons file.
# The font has hundreds of glyphs when I really only need a handlful...
# Removing unneeded ones saves lots of space for the SVG build since I embed the font directly
#
# Source: https://stackoverflow.com/questions/64614572/creating-a-material-icons-subset

# The icons to keep in the font
ICONS="home browse book terminal sdk settings"

# Folder locations
ROOT=$(dirname $0)/..
FONTS=$ROOT/fonts

# Source of the Material Symbols file
# Attach `.woff2` for the font, or `.codepoints` for the codepoints
FONT_SRC="https://raw.githubusercontent.com/google/material-design-icons/master/variablefont/MaterialSymbolsOutlined%5BFILL%2CGRAD%2Copsz%2Cwght%5D"

# Download latest font
echo "Updating font..."
curl -s "$FONT_SRC.woff2" -o $FONTS/material-icons.woff2

# A file with conversions between icon names and codepoints
CODEPOINTS_SRC="$(curl -s $FONT_SRC.codepoints)"
CODEPOINTS=""

echo "Trimming font..."

# Convert icon names to codepoints
while IFS= read -r line
do
    NAME=$(cut -d ' ' -f 1 <<< "$line")
    echo $ICONS | grep -w -q $NAME
    if [ $? -eq 0 ]
    then
        CODEPOINT=$(cut -d ' ' -f 2 <<< "$line")
        echo "  Font icon: Name: '$NAME' // Codepoint: '$CODEPOINT'"
        CODEPOINTS="$CODEPOINT,$CODEPOINTS"
    fi
done <<< "$CODEPOINTS_SRC"

fonttools subset \
    "$FONTS/material-icons.woff2" \
    --unicodes="5f-7a,30-39,$CODEPOINTS" \
    --no-layout-closure \
    --flavor=woff2 \
    --output-file=$FONTS/material-icons.trimmed.woff2
