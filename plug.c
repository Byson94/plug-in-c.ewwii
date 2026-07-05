#include "ewwii.h"
#include <stdio.h>
#include <inttypes.h>

RawMetadata plugin_metadata(void) {
    RawMetadata info;
    info.id = "my-awesome-plugin";
    info.version = "1.0.0";
    return info;
}

void test_func(const struct HostHandle *host, uint64_t *val) {
    printf("Testing! hi Testing\n");
    printf("%" PRIu64 "\n", *val);
    fflush(stdout);
}

void plugin_init(const struct HostHandle *host) {
    ewwii_log(host, "Hello from C!");
    ewwii_warn(host, "Warning from C!");
    ewwii_error(host, "Error from C!");

    ewwii_inject_css(host, "* { all: unset }", test_func);
    ewwii_inject_nbcl(host, "let secret = '123xyz'");
}
