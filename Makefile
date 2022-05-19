ROMS_LINK := "https://projects.intra.42.fr/uploads/document/document/4986/roms.zip"
ROMS := \
	assets/roms/Super\ Mario\ Land.gb \
	assets/roms/Legend\ of\ Zelda,\ The\ -\ Link's\ Awakening\ DX.gbc \
	assets/roms/Legend\ of\ Zelda,\ The\ -\ Oracle\ of\ Seasons.gbc \
	assets/roms/Pokemon_Rouge.gb \
	assets/roms/Pokemon\ -\ Version\ Cristal.gbc \
	assets/roms/Tetris.gb \
	assets/roms/Bubble_Ghost.gb \
	assets/roms/Pokemon\ -\ Version\ Argent.gbc \
	assets/roms/Super\ Mario\ Land\ 2.gb \
	assets/roms/Metroid\ II\ -\ Return\ of\ Samus.gb \
	assets/roms/Pokemon\ -\ Version\ Or.gbc \
	assets/roms/Legend_of_Zelda_link_Awaking.gb \
	assets/roms/Metal\ Gear\ Solid.gbc \
	assets/roms/Kirby\ 2.gb \
	assets/roms/Mystic_Quest.gb \
	assets/roms/Pokemon_Bleue.gb \
	assets/roms/Legend\ of\ Zelda,\ The\ -\ Oracle\ of\ Ages.gbc \
	assets/roms/Pokemon\ -\ Jaune.gbc \

BIOS_LINK_ROOT := "https://gbdev.gg8.se/files/roms/bootroms"
BIOS := \
		assets/bios/dmg_boot.bin \
		assets/bios/cgb_boot.bin \

TARGET_DIR ?= target
ASSETS_DIR := assets
ROMS_ZIP := $(ASSETS_DIR)/roms.zip
ROMS_DIR := $(ASSETS_DIR)/roms
BIOS_DIR := $(ASSETS_DIR)/bios

ifneq ($(OS),Windows_NT)
UNAME_S := $(shell uname -s)
endif

requirement: roms bios

bios: $(BIOS)

$(BIOS_DIR)/%: $(ASSETS_DIR)/bios.zip
	# curl --create-dirs --output $@ $(addprefix $(BIOS_LINK_ROOT)/, $*)
	unzip -n $< "$*" -d $(BIOS_DIR)
	touch "$@"

roms: $(ROMS)

$(ROMS_ZIP):
	curl --output $@ $(ROMS_LINK)

$(ROMS_DIR)/%: $(ROMS_ZIP)
	unzip -n $< "roms/$*" -d $(ASSETS_DIR)
	touch "$@"

docker: Dockerfile packaging/linux/appimage/Dockerfile
	docker build -f Dockerfile -t gbmu:latest .
	docker build -f packaging/linux/appimage/Dockerfile -t gbmu-appimage:latest .

run-container: docker
	docker run -it --net=host --env=DISPLAY --rm gbmu:latest

package-linux: package-linux-appimage

package-linux-appimage: docker
	mkdir -p build

	docker run --name=build-gbmu -t -v $$(pwd)/build:/build --entrypoint=/bin/sh gbmu-appimage:latest -c "set -x && appimage-builder --skip-tests && zip -r GBMU.AppDir.zip GBMU.AppDir"
	docker commit build-gbmu build-gbmu-img
	docker container rm build-gbmu

	docker run --name build-gbmu-pkg --entrypoint=sleep -d build-gbmu-img 3600
	docker cp build-gbmu-pkg:/home/tester/GBMU-latest-x86_64.AppImage ./build/GBMU-latest-x86_64.AppImage
	docker cp build-gbmu-pkg:/home/tester/GBMU.AppDir.zip ./build/GBMU.AppDir.zip

	docker kill build-gbmu-pkg
	docker container rm build-gbmu-pkg

package-linux-simple: gbmu.zip

gbmu.zip: $(ASSETS_DIR)/gbmu-512x512.png packaging/linux/simple/gbmu.desktop packaging/linux/simple/install.sh $(TARGET_DIR)/release/gbmu
	zip -j $@ $^

$(TARGET_DIR)/release/gbmu:
	cargo build --release

package-mac:
	cargo build --release
	./packaging/mac/package.sh target/release/gbmu GBMU

ifeq ($(OS),Windows_NT)
package:
	@echo "Build on windows not supported (yet ?)"
else ifeq ($(UNAME_S),Linux)
package: package-linux
else ifeq ($(UNAME_S),Darwin)
package: package-mac
endif

clean:
	rm -rf build/

.PHONY: requirement roms docker run-container package package-linux package-linux-appimage package-mac clean
