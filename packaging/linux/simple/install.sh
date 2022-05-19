#!/bin/bash

GBMU_FILENAME=gbmu
ICON_FILENAME=gbmu-512x512.png
DESKTOP_FILENAME=gbmu.desktop

GBMU_FILE=$GBMU_FILENAME
ICON_FILE=$ICON_FILENAME
DESKTOP_FILE=$DESKTOP_FILENAME


DESKTOP_INSTALL_DIR=$HOME/.local/share/applications
INSTALL_DIR=$HOME/.config/gbmu

if [ ! -f $GBMU_FILE ]; then
  echo "Missing executable to be installed" >&2
  exit 1
fi

if [ ! -f $ICON_FILE ]; then
  echo "Missing icon file" >&2
  exit 1
fi

if [ ! -f $DESKTOP_FILE ]; then
  echo "Missing desktop file" >&2
  exit 1
fi

mkdir -p $DESKTOP_INSTALL_DIR

echo "Creating install dir at $INSTALL_DIR"
mkdir -p $INSTALL_DIR

echo "Copying files to $INSTALL_DIR"
cp -v $GBMU_FILE $ICON_FILE $INSTALL_DIR/

sed -i.back \
  -e "s;^Exec=.*$;Exec=$INSTALL_DIR/$GBMU_FILENAME;" \
  -e "s;^Icon=.*$;Icon=$INSTALL_DIR/$ICON_FILENAME;" \
  $DESKTOP_FILE

desktop-file-install --dir=$DESKTOP_INSTALL_DIR $DESKTOP_FILE
update-desktop-database -v $DESKTOP_INSTALL_DIR
