#include <stdio.h>
#include "mylibrary.h"

int main() {

    // queue
    struct queue_head *head = create_queue_head();
    // head = (struct queue_head*)malloc(sizeof(struct queue_head));
    queue_init(head);
    long unsigned int i = 10;
    printf("PUSH 1: %lu\n", (unsigned long) queue_push(head, (void *)i));
    printf("PEEK 1: %lu\n", (unsigned long) queue_peek(head));
    printf("NUM: %d\n", head->num);
    printf("PEEK 2: %lu\n", (unsigned long) queue_peek(head));
    printf("POP 1: %lu\n", (unsigned long) queue_pop(head));
    // printf("POP 2: %lu\n", (unsigned long) queue_pop(head));
    // printf("PEEK 3: %lu\n", (unsigned long) queue_peek(head));


    return 0;
}