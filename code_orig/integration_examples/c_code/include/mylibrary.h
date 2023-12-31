#ifndef MYLIBRARY_H_
#define MYLIBRARY_H_

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

struct queue_entry;
struct queue_head;

struct queue_entry {
    struct queue_entry *next;
    void *data;
} queue_entry;

struct queue_head {
    struct queue_entry *head;
    struct queue_entry *tail;
    unsigned int num;
};

// uint32_t add(uint32_t, uint32_t);
uint32_t write_to_stderr(const char* message);
uint32_t hexdump(void* fp, const unsigned char* data, size_t data_len);

#endif