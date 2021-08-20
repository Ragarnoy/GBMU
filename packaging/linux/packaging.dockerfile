FROM gbmu:latest as BUILDER

RUN cargo build --release

FROM appimagecrafters/appimage-builder:latest

# Issue AppImage in docker: https://appimage-builder.readthedocs.io/en/latest/intro/install.html#install-appimagetool
RUN apt-get update && apt-get install -y wget \
  && set -x \
  && wget -q https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage -O /opt/appimagetool \
  && sh -c "set -x && cd /opt/ && chmod +x appimagetool && sed -i 's|AI\x02|\x00\x00\x00|' appimagetool && ./appimagetool --appimage-extract" \
  && mv -v /opt/squashfs-root /opt/appimagetool.AppDir \
  && ln -s /opt/appimagetool.AppDir/AppRun /usr/local/bin/appimagetool

RUN useradd tester -d /home/tester -m
USER tester
WORKDIR /home/tester

COPY --from=BUILDER --chown=tester:tester /app/target/release/gbmu /home/tester/GBMU.AppDir/usr/bin/gbmu
COPY --chown=tester:tester assets/gbmu.desktop assets/gbmu.png /home/tester/GBMU.AppDir/
COPY --chown=tester:tester assets/AppImageBuilder.yml /home/tester/

RUN appimage-builder
