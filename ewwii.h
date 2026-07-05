#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct HostHandle {
  const void *inner;
} HostHandle;

typedef struct RawMetadata {
  const char *id;
  const char *version;
} RawMetadata;

void ewwii_log(const struct HostHandle *handle, const char *msg);

void ewwii_warn(const struct HostHandle *handle, const char *msg);

void ewwii_error(const struct HostHandle *handle, const char *msg);

uint64_t *ewwii_inject_css(const struct HostHandle *handle, const char *css);

void ewwii_inject_nbcl(const struct HostHandle *handle, const char *nbcl);

void ewwii_register_signal(const struct HostHandle *handle, const char *name, const char *initial);

void ewwii_update_signal(const struct HostHandle *handle, const char *name, const char *value);

const char *ewwii_api_version(void);

extern struct RawMetadata plugin_metadata(void);

extern void plugin_init(const struct HostHandle *host);
