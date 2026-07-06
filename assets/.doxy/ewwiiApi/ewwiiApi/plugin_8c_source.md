

# File plugin.c

[**File List**](files.md) **>** [**plugc-ewwii**](dir_c0686db14fc2b0ffea2c0908b8a6159a.md) **>** [**test**](dir_dc67d04d6112947ffeac385740190ebf.md) **>** [**src**](dir_4785b39e4c916e9d8debc5856b2221b0.md) **>** [**plugin.c**](plugin_8c.md)

[Go to the documentation of this file](plugin_8c.md)


```C++
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
```


