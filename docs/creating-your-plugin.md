# Creating your plugin

Since you've finished [installing](getting-started.md) the C bindings, let's setup your plugin now.

You can start off by running this command:

```bash
plugc-ewwii init --name "awesome-plugin"
```

It will do all the hard work of setting up your plugin, Makefile, bindings, etc. After its done, you'll have 
a file strucutre like this:

```txt
.
├── bindings
│   ├── ewwii.h
│   └── libpluginc_ewwii.a
├── Makefile
└── src
    └── plugin.c
```

There exists an example plugin at `src/plugin.c` .You can run `make` you compile this plugin.
