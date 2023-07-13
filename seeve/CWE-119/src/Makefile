CC=emcc
CFLAGS=-O2

ifeq "$(CC)" "gcc"
  EXT=o
else
  EXT=wasm
endif

all:	test1 test2 test3 test4

test1:
	$(CC) $(CFLAGS) -o test1.$(EXT) test1.c 
	wasm2wat test1.$(EXT) > test1.wat
test2:
	$(CC) $(CFLAGS) -o test2.$(EXT) test2.c
	wasm2wat test2.$(EXT) > test2.wat
test3:
	$(CC) $(CFLAGS) -o test3.$(EXT) test3.c
	wasm2wat test3.$(EXT) > test3.wat
test4:
	$(CC) $(CFLAGS) -o test4.$(EXT) test4.c
	wasm2wat test4.$(EXT) > test4.wat
clean:
	rm test*.o test*.wasm test*.wat
