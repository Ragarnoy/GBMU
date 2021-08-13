#!/usr/bin/env bash

APPNAME=${2:-$(basename "${1}" '.sh')};
CONTENT="${APPNAME}.app/Contents";
RESOURCES="${CONTENT}/Resources";
MACOS="${CONTENT}/MacOS";

if [ -a "${APPNAME}.app" ]; then
	rm -rf "${APPNAME}.app"
fi;

mkdir -p "${MACOS}";
cp "${1}" "${MACOS}/${APPNAME}";
chmod +x "${MACOS}/${APPNAME}";

mkdir "${CONTENT}";
cp "Info.plist" "${CONTENT}/";

mkdir "${RESOURCES}";
# convert logo.png to the icon file format for macos, the commands sips might not be installed on every mac, simplier to embark the file
# sips -s format icns logo.png --out logo.icns
cp "logo.icns" "${RESOURCES}/";

echo "${PWD}/$APPNAME.app";
