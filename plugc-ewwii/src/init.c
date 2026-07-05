#include <unistd.h>
#include <limits.h>
#include <libgen.h>
#include <string.h>
#include <dirent.h>
#include <stdbool.h>
#include <unistd.h>

#include "init.h"

char* get_dirname() {
    char full_path[PATH_MAX];
    
    if (getcwd(full_path, sizeof(full_path)) != NULL) {
        char* base = basename(full_path);
        return base;
    } else {
        return NULL;
    }
}

bool dir_nonempty() {
    char path[PATH_MAX];
    
    if (getcwd(path, sizeof(path)) == NULL) {
        return false;
    }

    DIR *d = opendir(path);
    if (d == NULL) return false;

    struct dirent *entry;
    bool is_nonempty = false;

    while ((entry = readdir(d)) != NULL) {
        if (strcmp(entry->d_name, ".") != 0 && strcmp(entry->d_name, "..") != 0) {
            is_nonempty = true;
            break;
        }
    }

    closedir(d);
    return is_nonempty;
}

void init_example(const char* name) {

}

void init_makefile(const char* name) {

}

void init_bindings() {

}
