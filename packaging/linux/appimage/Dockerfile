FROM gbmu:latest as BUILDER

RUN cargo build --release

# FROM appimagecrafters/appimage-builder:latest
FROM ubuntu:21.10

# Issue AppImage in docker: https://appimage-builder.readthedocs.io/en/latest/intro/install.html#install-appimagetool
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
  wget \
  strace \
  appstream \
  python3-pip \
  python3-setuptools \
  patchelf \
  desktop-file-utils \
  libgdk-pixbuf2.0-dev \
  fakeroot \
  fuse \
  gtk-update-icon-cache

ADD https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage /usr/local/bin/appimagetool
RUN chmod a+x /usr/local/bin/appimagetool \
  && pip3 install appimage-builder

RUN useradd tester -d /home/tester -m
USER tester
WORKDIR /home/tester

COPY --chown=tester:tester assets/gbmu-512x512.png /home/tester/GBMU.AppDir/usr/share/icons/gbmu/512x512/gbmu.png
COPY --chown=tester:tester assets/gbmu-512x512.png /home/tester/GBMU.AppDir/usr/share/pixmaps/gbmu.png
COPY --chown=tester:tester assets/gbmu-512x512.png /home/tester/GBMU.AppDir/usr/share/icons/hicolor/512x512/gbmu.png

COPY --chown=tester:tester assets/gbmu.metainfo.xml /home/tester/GBMU.AppDir/usr/share/metainfo/gbmu.appdata.xml
COPY --chown=tester:tester packaging/linux/appimage/AppImageBuilder.yml /home/tester/

COPY --from=BUILDER --chown=tester:tester /app/target/release/gbmu /home/tester/GBMU.AppDir/usr/bin/gbmu

RUN appstreamcli make-desktop-file GBMU.AppDir/usr/share/metainfo/gbmu.appdata.xml GBMU.AppDir/usr/share/applications/gbmu.desktop
ENTRYPOINT appimage-builder
CMD [ "--skip-tests" ]
