#include "ewwii.h"

RawMetadata plugin_metadata(void) {
    RawMetadata info;
    info.id = "test";
    info.version = "1.0.0";
    return info;
}

void plugin_init(const struct HostHandle *host) {
    ewwii_log(host, "Hello from C!");
}