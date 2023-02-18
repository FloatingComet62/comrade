SRC = $(wildcard src/*.cpp)
CFLAGS = -Ofast

CPP = g++

debug: $(SRC)
	$(CPP) $(CFLAGS) $(SRC) -o debug.exe

release: $(SRC)
	$(CPP) $(CFLAGS) -O3 -Oz $(SRC) -o release.exe