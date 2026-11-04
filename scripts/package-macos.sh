#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-or-later
# Build a branded macOS Installer from an already verified CrabCut.app.
set -euo pipefail
if [[ $# -ne 4 ]]; then
  echo "usage: $0 /path/CrabCut.app version arm64|x86_64 /path/output.pkg" >&2
  exit 2
fi
app=$1
version=$2
arch=$3
output=$4
case "$arch" in arm64|x86_64) ;; *) echo "Unsupported architecture" >&2; exit 2;; esac
root=$(cd "$(dirname "$0")/.." && pwd)
staging=$(mktemp -d "${TMPDIR:-/tmp}/crabcut-package.XXXXXX")
trap 'rm -rf "$staging"' EXIT
mkdir -p "$staging/resources"
cp "$root/assets/macos/installer/"* "$staging/resources/"
cp "$root/LICENSE" "$staging/resources/LICENSE.txt"
pkgbuild --component "$app" --install-location /Applications \
  --identifier app.crabcut.editor --version "$version" "$staging/CrabCut-component.pkg"
python3 - "$staging" "$version" "$arch" "$app" <<'PY'
import sys,plistlib,xml.etree.ElementTree as ET
from pathlib import Path
folder,version,arch,app=sys.argv[1:]
with open(Path(app)/"Contents/Info.plist", "rb") as f:
    minimum=plistlib.load(f)["LSMinimumSystemVersion"]
root=ET.Element('installer-gui-script',{'minSpecVersion':'2'})
ET.SubElement(root,'title').text='CrabCut · Althera Solutions'
for tag in ('background', 'background-darkAqua'):
    ET.SubElement(root,tag,{'file':'sidebar.png','alignment':'bottomleft','scaling':'none','mime-type':'image/png'})
ET.SubElement(root,'welcome',{'file':'welcome.html','mime-type':'text/html'})
ET.SubElement(root,'license',{'file':'LICENSE.txt','mime-type':'text/plain'})
ET.SubElement(root,'conclusion',{'file':'conclusion.html','mime-type':'text/html'})
ET.SubElement(root,'options',{'customize':'never','require-scripts':'true','hostArchitectures':arch})
volume=ET.SubElement(root,'volume-check',{'script':'true'})
allowed=ET.SubElement(volume,'allowed-os-versions')
ET.SubElement(allowed,'os-version',{'min':minimum})
outline=ET.SubElement(root,'choices-outline');ET.SubElement(outline,'line',{'choice':'crabcut'})
choice=ET.SubElement(root,'choice',{'id':'crabcut','visible':'false','title':'CrabCut'})
ET.SubElement(choice,'pkg-ref',{'id':'app.crabcut.editor'})
ET.SubElement(root,'pkg-ref',{'id':'app.crabcut.editor','version':version,'onConclusion':'none'}).text='CrabCut-component.pkg'
ET.ElementTree(root).write(Path(folder)/'distribution.xml',encoding='utf-8',xml_declaration=True)
PY
productbuild --distribution "$staging/distribution.xml" --package-path "$staging" \
  --resources "$staging/resources" "$output"
pkgutil --check-signature "$output" || true
