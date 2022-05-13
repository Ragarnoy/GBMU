FROM gbmu:latest as BUILDER

RUN cargo build --release

# FROM appimagecrafters/appimage-builder:latest
FROM ubuntu:22.04

# Issue AppImage in docker: https://appimage-builder.readthedocs.io/en/latest/intro/install.html#install-appimagetool
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
  wget \
  strace \
  appstream \
  python3-pip \
  python3-setuptools \
  python3 \
  patchelf \
  desktop-file-utils \
  libgdk-pixbuf2.0-dev \
  fakeroot \
  fuse \
  gtk-update-icon-cache \
  file \
  elfutils \
  libgtk-3-0 \
  zip

WORKDIR /opt
ADD https://github.com/AppImage/AppImageKit/releases/download/13/appimagetool-x86_64.AppImage /opt/appimagetool
RUN chmod +x /opt/appimagetool \
  && /opt/appimagetool --appimage-extract \
  && mv -v squashfs-root appimage-tool.AppDir \
  && ln -s /opt/appimage-tool.AppDir/AppRun /usr/bin/appimagetool \
  && pip3 install appimage-builder==0.9.2

RUN useradd tester -d /home/tester -m
USER tester
WORKDIR /home/tester

COPY --chown=tester:tester assets/gbmu-512x512.png /home/tester/GBMU.AppDir/usr/share/icons/gbmu/512x512/gbmu.png
COPY --chown=tester:tester assets/gbmu-512x512.png /home/tester/GBMU.AppDir/usr/share/pixmaps/gbmu.png
COPY --chown=tester:tester assets/gbmu-512x512.png /home/tester/GBMU.AppDir/usr/share/icons/hicolor/512x512/gbmu.png

COPY --chown=tester:tester assets/gbmu.metainfo.xml /home/tester/GBMU.AppDir/usr/share/metainfo/io.github.ragarnoy.GBMU.appdata.xml
COPY --chown=tester:tester packaging/linux/appimage/AppImageBuilder.yml /home/tester/

COPY --from=BUILDER --chown=tester:tester /app/target/release/gbmu /home/tester/GBMU.AppDir/usr/bin/gbmu

RUN mkdir -p GBMU.AppDir/usr/share/applications \
  && appstreamcli make-desktop-file GBMU.AppDir/usr/share/metainfo/io.github.ragarnoy.GBMU.appdata.xml GBMU.AppDir/usr/share/applications/io.github.ragarnoy.GBMU.desktop

ENTRYPOINT appimage-builder
CMD [ "--skip-tests" ]
