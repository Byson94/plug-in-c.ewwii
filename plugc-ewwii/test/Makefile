CC = gcc
CFLAGS = -fPIC -Wall -I bindings
LDFLAGS = -shared -ldl -lm -lrt

PLUGIN_OUT = libtest.so
RUST_BRIDGE = bindings/libpluginc_ewwii.a
SOURCES = $(wildcard src/*.c)

all: $(PLUGIN_OUT)

$(PLUGIN_OUT): $(SOURCES)
	$(CC) $(CFLAGS) -o $@ $^ $(RUST_BRIDGE) $(LDFLAGS)

clean:
	rm $(PLUGIN_OUT)

.PHONY: all clean