#!/bin/bash

# Script to automatically fix corrupted Cursor extensions
# This removes corrupted extensions and reinstalls them

CURSOR_EXT_DIR="$HOME/.cursor/extensions"

echo "Checking for corrupted extensions..."

# Find all extension directories missing package.json
corrupted=()
for dir in "$CURSOR_EXT_DIR"/*/; do
    if [ -d "$dir" ] && [ ! -f "$dir/package.json" ]; then
        ext_name=$(basename "$dir")
        corrupted+=("$ext_name")
        echo "Found corrupted extension: $ext_name"
    fi
done

if [ ${#corrupted[@]} -eq 0 ]; then
    echo "No corrupted extensions found!"
    exit 0
fi

echo ""
echo "Found ${#corrupted[@]} corrupted extension(s)"
echo ""

# Extract extension IDs from directory names
# Format: publisher.name-version or publisher.name-version-platform
ext_ids=()
for corrupted_ext in "${corrupted[@]}"; do
    # Remove version and platform suffixes to get extension ID
    # e.g., "ms-python.python-2025.6.1-darwin-arm64" -> "ms-python.python"
    ext_id=$(echo "$corrupted_ext" | sed -E 's/-[0-9]+\.[0-9]+\.[0-9]+.*$//')
    ext_ids+=("$ext_id")
done

# Remove duplicates
unique_ext_ids=($(printf "%s\n" "${ext_ids[@]}" | sort -u))

echo "Removing corrupted extension directories..."
for corrupted_ext in "${corrupted[@]}"; do
    echo "  Removing: $corrupted_ext"
    rm -rf "$CURSOR_EXT_DIR/$corrupted_ext"
done

echo ""
echo "Reinstalling extensions..."
for ext_id in "${unique_ext_ids[@]}"; do
    echo "  Reinstalling: $ext_id"
    cursor --install-extension "$ext_id" 2>&1 | grep -E "(successfully|already|Error)" || true
done

echo ""
echo "Updating all extensions..."
cursor --update-extensions 2>&1 | grep -E "(successfully|Error)" || true

echo ""
echo "Done! Please restart Cursor to verify all extensions are working."

