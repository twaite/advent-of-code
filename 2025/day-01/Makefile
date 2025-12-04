.PHONY: all build clean run test

all: build

build:
	cmake -B build -G Ninja
	ninja -C build

clean:
	rm -rf build

run: build
	./build/day-01

test: build
	ninja -C build test