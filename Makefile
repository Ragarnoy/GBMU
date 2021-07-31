ROMS_LINK := "https://projects.intra.42.fr/uploads/document/document/2833/roms.zip"
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

ROMS_DIR := roms

requirement: roms

roms: $(ROMS)


roms.zip:
	wget $(ROMS_LINK) -O $@

$(ROMS_DIR)/%: roms.zip
	echo "target: $@"
	unzip $< 'roms/*' -x '*/.DS_Store'
	touch roms/*

CURRENT_OS=
ifeq ($(OS),Windows_NT)
	CURRENT_OS := windows
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		CURRENT_OS := Linux
	endif
	ifeq ($(UNAME_S),Darwin)
		CURRENT_OS := MacOS
	endif
endif

extern_dependencies:
ifeq ($(CURRENT_OS),Linux)
	sudo apt install cmake libgtk-3-dev
endif
ifeq ($(CURRENT_OS),MacOS)
	brew install cmake
endif
ifeq ($(CURRENT_OS),Windows)
	echo missing dependencies on windows
endif

.PHONY: requirement roms extern_dependencies
