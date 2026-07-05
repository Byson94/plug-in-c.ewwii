#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>
#include <stdint.h>
#include <stddef.h>

void ffi_gateway(const uint8_t *ptr, size_t len) {
    // This will be defined in ewwii
}

int main(int argc, char** argv) {
    if (argc <= 1) {
        printf("Usage: loader <PATH/TO/PLUG.so>\n");
    }

    printf("Loading plugin...\n");
    char *path = argv[1];
    void *plugin = dlopen(path, RTLD_LAZY);

    if (!plugin) {
        fprintf(stderr, "Library load error: %s\n", dlerror());
        exit(EXIT_FAILURE);
    }

    printf("Plugin loaded!\n");

    typedef const char* (*get_version_func)(void);
    get_version_func get_version = (get_version_func)dlsym(plugin, "ewwii_api_version");

    if (get_version == NULL) {
        fprintf(stderr, "Symbol not found: %s\n", dlerror());
        return 1;
    }

    const char* version = get_version();
    printf("Plugin Version: %s\n", version);

    return 0;
}
