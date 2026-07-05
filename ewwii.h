#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct HostHandle {
  const void *inner;
} HostHandle;

typedef struct RuntimePaths {
  const char *log_file;
  const char *log_dir;
  const char *ipc_socket_file;
  const char *config_dir;
} RuntimePaths;

typedef void (*CListenCallback)(const char*, const char*);

typedef struct RawMetadata {
  const char *id;
  const char *version;
} RawMetadata;

void ewwii_log(const struct HostHandle *handle, const char *msg);

void ewwii_warn(const struct HostHandle *handle, const char *msg);

void ewwii_error(const struct HostHandle *handle, const char *msg);

uint64_t *ewwii_inject_css(const struct HostHandle *handle, const char *css);

void ewwii_remove_css(const struct HostHandle *handle, uint64_t *idx_ptr);

void ewwii_inject_nbcl(const struct HostHandle *handle, const char *nbcl);

const struct RuntimePaths *ewwii_get_runtime_paths(const struct HostHandle *handle);

void ewwii_emit(const struct HostHandle *handle, const char *signal, const char *data);

void ewwii_listen(const struct HostHandle *handle, const char *signal, CListenCallback callback);

void ewwii_register_signal(const struct HostHandle *handle, const char *name, const char *initial);

void ewwii_update_signal(const struct HostHandle *handle, const char *name, const char *value);

const char *ewwii_signal_value(const struct HostHandle *handle, const char *name);

const char *ewwii_api_version(void);

extern struct RawMetadata plugin_metadata(void);

extern void plugin_init(const struct HostHandle *host);
