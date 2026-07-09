# Examples

Examples of common plugin patterns.

## Injecting CSS 

Example of injecting and removing css.

```c
// is ran when css is injected and id is resolved
void on_inject(const struct HostHandle *host, uint64_t *id) {
    // remove the css immediately
    ewwii_remove_css(host, id);
}

void plugin_init(const struct HostHandle *host) {
    ewwii_inject_css(host, "* { all: unset }", on_inject);
}
```

## Emissions & Listening

```c
void handle_listen(const char *signal, const char *data) {
    printf("signal encountered: %s\n", signal);
    printf("signal data: %s\n", data);
    fflush(stdout); // important!
}

void plugin_int(const struct HostHandle *host) {
    // emitting message for other plugins
    ewwii_emit(host, "this-plugin-loaded", "<none>");

    // listening to events
    ewwii_listen(host, "ewwii-init-window", handle_listen);
}
```
