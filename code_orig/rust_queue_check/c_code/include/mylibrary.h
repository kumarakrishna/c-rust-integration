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

void queue_init (struct queue_head *queue);
struct queue_head* create_queue_head();
void *queue_push(struct queue_head *queue, void *data);
void *queue_pop(struct queue_head *queue);
void *queue_peek(struct queue_head *queue);

#endif