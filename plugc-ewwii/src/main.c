#include <stdio.h>
#include <string.h>

#include "args.h"
#include "init.h"

int main(int argc, char** argv) {
    if (argc <= 1) {
        print_help();
        return 0;
    }

    char* cmd = argv[1];
    if (strcmp(cmd, "init") == 0) {
        if (dir_nonempty()) {
            printf("Refusing to operate on a non-empty directory.\n");
            return 1;
        }

        char* name;
        if (argc <= 2) {
            char* dirname = get_dirname();
            if (dirname == NULL) {
                printf("Could not determine directory name.\n");
                return 1;
            } else {
                name = get_dirname();
            }
        } else {
            char* flag = argv[2];
            if (strcmp(flag, "--name") == 0) {
                if (argc <= 3) {
                    printf("'--name' flag requires a following string\n");
                    return 1;
                }

                name = argv[3];
            } else {
                char* dirname = get_dirname();
                if (dirname == NULL) {
                    printf("Could not determine directory name.\n");
                    return 1;
                } else {
                    name = get_dirname();
                }
            }
        }

        printf("Initializing...\n");
        printf("Initializing Example...\n");
        init_example(name);

        printf("Initializing Makefile...\n");
        init_makefile(name);

        printf("Initializing Bindings...\n");
        init_bindings();
    }
    
    return 0;
}
