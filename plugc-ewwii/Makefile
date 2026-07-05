CC = gcc
CFLAGS = -Iinclude
LDFLAGS = -lcurl

OUT = plugc-ewwii
SOURCES = $(wildcard src/*.c)

all: $(OUT)

$(OUT): $(SOURCES)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

clean: 
	rm $(OUT)

.PHONY: all clean
