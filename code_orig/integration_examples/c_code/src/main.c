#include <stdio.h>
#include "mylibrary.h"

int main() {

    const char* message = "This is a message from C written by rust.";
    int result = write_to_stderr(message);

    if (result == 0) {
        printf("Write to stderr successful.\n");
    } else {
        printf("Error writing to stderr.\n");
    }

    FILE *file = fopen("./bin/output.txt", "w");
    if (file == NULL) {
        perror("Error opening file");
        return 1;
    }

    const unsigned char data[] = "This is a test string for hexdump function.";
    size_t data_len = sizeof(data) - 1;
    result = hexdump(file, data, data_len);

    if (result != 0) {
        printf("hexdump failed with error code %d\n", result);
    } else {
        printf("hexdump completed successfully.\n");
    }

    return 0;
}