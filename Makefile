ROMS_LINK := "https://projects.intra.42.fr/uploads/document/document/4986/roms.zip"
ROMS := \
	roms/Super\ Mario\ Land.gb \
	roms/Legend\ of\ Zelda,\ The\ -\ Link's\ Awakening\ DX.gbc \
	roms/Legend\ of\ Zelda,\ The\ -\ Oracle\ of\ Seasons.gbc \
	roms/Pokemon_Rouge.gb \
	roms/Pokemon\ -\ Version\ Cristal.gbc \
	roms/Tetris.gb \
	roms/Bubble_Ghost.gb \
	roms/Pokemon\ -\ Version\ Argent.gbc \
	roms/Super\ Mario\ Land\ 2.gb \
	roms/Metroid\ II\ -\ Return\ of\ Samus.gb \
	roms/Pokemon\ -\ Version\ Or.gbc \
	roms/Legend_of_Zelda_link_Awaking.gb \
	roms/Metal\ Gear\ Solid.gbc \
	roms/Kirby\ 2.gb \
	roms/Mystic_Quest.gb \
	roms/Pokemon_Bleue.gb \
	roms/Legend\ of\ Zelda,\ The\ -\ Oracle\ of\ Ages.gbc \
	roms/Pokemon\ -\ Jaune.gbc \

BIOS_LINK_ROOT := "https://gbdev.gg8.se/files/roms/bootroms"
BIOS := \
		roms/bios/dmg_boot.bin \
		roms/bios/cgb_boot.bin \

ROMS_DIR := roms

requirement: roms bios

bios: $(BIOS)

roms/bios/%:
	curl --create-dirs --output $@ $(addprefix $(BIOS_LINK_ROOT)/, $*)

roms: $(ROMS)

roms.zip:
	curl --output $@ $(ROMS_LINK)

$(ROMS_DIR)/%: roms.zip
	echo "target: $@"
	unzip $< 'roms/*' -x '*/.DS_Store'
	# touch roms/*

docker: Dockerfile packaging/linux/appimage/Dockerfile
	docker build -f Dockerfile -t gbmu:latest .
	docker build -f packaging/linux/appimage/Dockerfile -t gbmu-appimage:latest .

run-container: docker
	docker run -it --net=host --env=DISPLAY --rm gbmu:latest

package-linux: package-linux-appimage

package-linux-appimage: docker
	mkdir -p build
	docker run --rm -t -v $$(pwd)/build:/build --entrypoint=/bin/sh gbmu-appimage:latest -c "set -x && appimage-builder --skip-tests && zip -r GBMU.AppDir.zip GBMU.AppDir && cp -vR GBMU-latest-x86_64.AppImage GBMU.AppDir.zip /build/"

ifneq ($(OS),Windows_NT)
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Darwin)
package-mac:
	cargo build --release
	./packaging/mac/package.sh target/release/gbmu GBMU
endif

ifeq ($(OS),Windows_NT)
package:
	@echo "Build on windows not supported (yet ?)"
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
package: package-linux
    endif
    ifeq ($(UNAME_S),Darwin)
package: package-mac
    endif
endif

clean:
	rm -rf build/

.PHONY: requirement roms docker run-container package package-linux package-linux-appimage package-mac clean
