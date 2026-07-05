#pragma once

#include <stdbool.h>

char* get_dirname();
bool dir_nonempty();

void init_example(const char* name);
void init_makefile(const char* name);
void init_bindings();
