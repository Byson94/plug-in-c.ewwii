#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Host handle required for calling back to ewwii.
 */
typedef struct HostHandle {
  const void *inner;
} HostHandle;

/**
 * Paths to important dirs/files like confiuration directory.
 */
typedef struct CRuntimePaths {
  const char *log_file;
  const char *log_dir;
  const char *ipc_socket_file;
  const char *config_dir;
} CRuntimePaths;

typedef void (*CListenCallback)(const char*, const char*);

/**
 * Metadata of the plugin to register.
 */
typedef struct RawMetadata {
  const char *id;
  const char *version;
} RawMetadata;

/**
 * Log a message to ewwii with the appropriate plugin ID visible.
 *
 * @param handle The host handle 
 * @param msg The message to log
 */
void ewwii_log(const struct HostHandle *handle, const char *msg);

/**
 * Log a warning to ewwii with the appropriate plugin ID visible.
 *
 * @param handle The host handle 
 * @param msg The message to log
 */
void ewwii_warn(const struct HostHandle *handle, const char *msg);

/**
 * Log an error to ewwii with the appropriate plugin ID visible.
 *
 * @param handle The host handle 
 * @param msg The message to log
 */
void ewwii_error(const struct HostHandle *handle, const char *msg);

/**
 * Inject CSS into the core ewwii engine and handle the resulting CSS ID.
 *
 * @param handle The host handle 
 * @param css The css string to inject 
 * @future_handler A function to call when the CSS ID is resolved
 */
void ewwii_inject_css(const struct HostHandle *handle,
                      const char *css,
                      void (*future_handler)(const struct HostHandle*, uint64_t*));

/**
 * Remove an injected CSS from ewwii using the resolved CSS ID.
 *
 * @param handle The host handle 
 * @param idx_ptr The pointer to the resolved CSS ID which is to be removed
 */
void ewwii_remove_css(const struct HostHandle *handle, uint64_t *idx_ptr);

/**
 * Inject nbcl into ewwii.
 *
 * @param handle The host handle 
 * @param nbcl The nbcl code to inject
 */
void ewwii_inject_nbcl(const struct HostHandle *handle, const char *nbcl);

/**
 * Get the runtime paths like the configuration directory, socket file, etc.
 *
 * @param handle The host handle 
 * @future_handler The function to call when the CRuntimePaths are resolved
 */
void ewwii_get_runtime_paths(const struct HostHandle *handle,
                             void (*future_handler)(const struct HostHandle*,
                                                    const struct CRuntimePaths*));

/**
 * Emit a message which other plugins can see and work with the provided data.
 *
 * @param handle The host handle 
 * @signal The signal to emit 
 * @data The data to attach with the signal
 */
void ewwii_emit(const struct HostHandle *handle, const char *signal, const char *data);

/**
 * Listen to emissions made by other plugins and ewwii itself.
 *
 * @param handle The host handle
 * @param signal The signal to listen to 
 * @param callback The function to call when emission is found
 */
void ewwii_listen(const struct HostHandle *handle, const char *signal, CListenCallback callback);

/**
 * Register a signal (GlobalVar) to ewwii which can be accessed from configuration.
 *
 * @param handle The host handle 
 * @param name The name of the signal 
 * @param initial The initial value of the signal
 */
void ewwii_register_signal(const struct HostHandle *handle, const char *name, const char *initial);

/**
 * Update the value of a signal (Global).
 *
 * @param handle The host handle 
 * @param name The name of the signal to update
 * @param value The value to set
 */
void ewwii_update_signal(const struct HostHandle *handle, const char *name, const char *value);

/**
 * Get the value of a signal (GlobalVar) and do callback.
 *
 * @param handle The host handle 
 * @param name The name of the signal to get value of 
 * @param future_handler The function to call back to after resolving value
 */
void ewwii_signal_value(const struct HostHandle *handle,
                        const char *name,
                        void (*future_handler)(const struct HostHandle*, const char*));

const char *ewwii_api_version(void);

extern struct RawMetadata plugin_metadata(void);

extern void plugin_init(const struct HostHandle *host);
