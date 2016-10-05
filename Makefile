all: download build

build:
	cargo build

download:
	@svn export --force https://github.com/CleverRaven/Cataclysm-DDA/trunk/data/json/recipes vendor/cdda/recipes
