CC=emcc
CFLAGS=-O2

ifeq "$(CC)" "gcc"
  EXT=o
else
  EXT=wasm
endif

all:	test1 test2

test1:
	$(CC) $(CFLAGS) -o test1.$(EXT) test1.c
	wasm2wat test1.$(EXT) > test1.wat
test2:
	$(CC) $(CFLAGS) -o test2.$(EXT) test2.c
	wasm2wat test2.$(EXT) > test2.wat

clean:
	rm test*.o test*.wasm test*.wat
