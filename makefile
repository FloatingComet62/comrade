SRC = $(wildcard src/*.cpp)
CFLAGS = -Ofast -std=c++20

CPP = clang++

debug: $(SRC)
	$(CPP) $(CFLAGS) $(SRC) -o debug.exe

release: $(SRC)
	$(CPP) $(CFLAGS) -O3 -Oz $(SRC) -o release.exe