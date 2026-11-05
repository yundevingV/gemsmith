#!/usr/bin/env bash
# Build Gemsmith.app + CLI into a compressed DMG (macOS only).
set -euo pipefail

OUT="${1:?usage: macos-dmg.sh output.dmg}"
ROOT="$(mktemp -d)"
trap 'rm -rf "$ROOT"' EXIT

VERSION="$(grep '^version' Cargo.toml | head -1 | sed -E 's/version = "(.*)"/\1/')"
APP="Gemsmith.app"
STAGE="$ROOT/stage"
mkdir -p "$STAGE/$APP/Contents/MacOS"

cp target/release/gemsmith "$STAGE/$APP/Contents/MacOS/"
chmod +x "$STAGE/$APP/Contents/MacOS/gemsmith"

cat > "$STAGE/$APP/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>gemsmith</string>
  <key>CFBundleIdentifier</key>
  <string>com.gemsmith.app</string>
  <key>CFBundleName</key>
  <string>Gemsmith</string>
  <key>CFBundleDisplayName</key>
  <string>Gemsmith</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>${VERSION}</string>
  <key>CFBundleVersion</key>
  <string>${VERSION}</string>
  <key>NSHighResolutionCapable</key>
  <true/>
  <key>LSMinimumSystemVersion</key>
  <string>11.0</string>
</dict>
</plist>
PLIST

cp target/release/gemsmith-cli "$STAGE/"
ln -s /Applications "$STAGE/Applications"

hdiutil create \
  -volname "Gemsmith ${VERSION}" \
  -srcfolder "$STAGE" \
  -ov \
  -format UDZO \
  "$OUT"

echo "Created $OUT"
