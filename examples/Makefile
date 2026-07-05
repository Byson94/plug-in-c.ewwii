CC = gcc
CFLAGS = -fPIC -Wall
LDFLAGS = -shared -ldl -lm -lrt

PLUGIN_OUT = libplugin.so
RUST_BRIDGE = bindings/libpluginc_ewwii.a
SOURCES = $(wildcard src/*.c)

all: $(PLUGIN_OUT)

$(PLUGIN_OUT): $(SOURCES)
    $(CC) $(CFLAGS) -o $@ $^ $(RUST_BRIDGE) $(LDFLAGS)

clean:
    rm $(PLUGIN_OUT)

.PHONY all clean
