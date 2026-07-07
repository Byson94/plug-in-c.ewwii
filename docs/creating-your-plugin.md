# Creating your plugin

Once you finish [installing plugc-ewwii](getting-started.md), you can use it to setup your plugin.

## Initialization

You can start off by running this command:

```bash
plugc-ewwii init --name "awesome-plugin"
```

It will do all the hard work of setting up your plugin, Makefile, bindings, and other sorts of things. 
Once its done, you'll have a file structure like this:

```txt
.
├── bindings
│   ├── ewwii.h
│   └── libpluginc_ewwii.a
├── Makefile
└── src
    └── plugin.c
```

There exists an example plugin at `src/plugin.c`. You can run `make` you compile this plugin.

## Basics

`src/plugin.c` would look something like this:

```c 
#include "ewwii.h"

RawMetadata plugin_metadata(void) {
    RawMetadata info;
    info.id = "awesome-plugin";
    info.version = "1.0.0";
    return info;
}

void plugin_init(const struct HostHandle *host) {
    ewwii_log(host, "Hello from C!");
}
```

There are a few things going on here that you have to do to make your plugin work. All plugins **must** 
have a `plugin_metadata` and `plugin_init` function defined. 

Here is what they are used for:

- `plugin_metadata`: To register your plugin under an ID.
- `plugin_init`: To start executing your plugin.

You can mostly leave `plugin_metadata` as-is as you would mostly only ever need to change the `info.id`
or the `info.version` assignments. And in `plugin_init`, you can just implement your plugin logic by calling 
ewwii API.

So that's pretty much it. You can go to [C API Reference](api.md) to checkout the API's you can call or 
go to [Examples](examples.md) to see a few example plugins.
