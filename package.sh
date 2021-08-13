#!/usr/bin/env bash

APPNAME=${2:-$(basename "${1}" '.sh')};
DIR="${APPNAME}.app/Contents/MacOS";

if [ -a "${APPNAME}.app" ]; then
	rm -rf "${APPNAME}.app"
fi;

mkdir -p "${DIR}";
cp "${1}" "${DIR}/${APPNAME}";
chmod +x "${DIR}/${APPNAME}";

echo "${PWD}/$APPNAME.app";
