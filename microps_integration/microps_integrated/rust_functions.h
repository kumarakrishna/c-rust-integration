// rust_functions.h

#ifndef RUST_FUNCTIONS_H
#define RUST_FUNCTIONS_H

#include <stdint.h>
#include <stdio.h>

uint16_t ntoh16_rust(uint16_t n);
uint32_t ntoh32_rust(uint32_t n);
uint16_t cksum16_rust(uint16_t *data, size_t len, int sum);
void hexdump_rust(FILE *fp, const uint8_t *data, size_t len);

#endif
