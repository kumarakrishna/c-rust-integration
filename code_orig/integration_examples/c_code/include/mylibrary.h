#ifndef MYLIBRARY_H_
#define MYLIBRARY_H_

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

// uint32_t add(uint32_t, uint32_t);
uint32_t write_to_stderr(const char* message);
uint32_t hexdump(void* fp, const unsigned char* data, size_t data_len);

#endif