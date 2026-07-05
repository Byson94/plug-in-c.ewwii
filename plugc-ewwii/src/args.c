#include <stdio.h>

#include "args.h"

char* COMMANDS[1][2] = {
    { "init", "Initialize a new c plugin" },
};

char* FLAGS[1][2] = {
    { "--name", "Set name of the plugin (init)" },
};

void print_help() {
    printf("plugc-ewwii - C plugin bindings manager\n");
    printf("Usage: plugc-ewwii <COMMAND> <FLAGS>\n\n");
    printf("Commands:\n");

    int all_cmds = sizeof(COMMANDS)/sizeof(COMMANDS[0]);
    for (int i = 0; i < all_cmds; i++) {
        printf("  %-6s - %s\n", COMMANDS[i][0], COMMANDS[i][1]);
    }

    printf("\nFlags:\n");

    int all_flags = sizeof(FLAGS)/sizeof(FLAGS[0]);
    for (int i = 0; i < all_flags; i++) {
        printf("  %-6s - %s\n", FLAGS[i][0], FLAGS[i][1]);
    }
}
