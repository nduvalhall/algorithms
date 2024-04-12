#include <stdlib.h>
#include <stdio.h>


typedef struct {
    int length;
    int capacity;
    int* buf;
} array_t;


array_t* array_new(int capacity) {
    array_t* array = (array_t*)malloc(sizeof(array_t));
    
    if (array == NULL) {
        return NULL;
    }

    int* buf = (int*)malloc(sizeof(int)*capacity);

    if (buf == NULL) {
        free(array);
        return NULL;
    }

    array->length = 0;
    array->capacity = capacity;
    array->buf = buf;

    return array;
}


int array_insert(array_t* array, int value, int index) {
    if (array == NULL) {
        return 1;
    }

    if (array->length >= array->capacity) {
        array->capacity = ((array->capacity * 3) / 2) + 1;

        int* buf = (int*)realloc(array->buf, sizeof(int) * array->capacity);

        if (buf == NULL) {
            free(array);
            return 1;
        }

        array->buf = buf;
    }

    int new = value;

    for (int i = index; i <= array->length; i++) {
        int old = array->buf[i];
        array->buf[i] = new;
        new = old;
    }

    array->length++;
    return 0;
}


int array_delete(array_t* array, int index) {
    if (index >= array->length) {
        return 1;
    }

    if (index < 0) {
        return 1;
    }

    for (int i = index; i < array->length; i++) {
        array->buf[i] = array->buf[i + 1];       
    }

    array->length--;

    if (array->length < (array->capacity * 2) / 3) {
        array->capacity = (array->capacity * 2 ) / 3;

        int* buf = (int*)realloc(array->buf, sizeof(int) * array->capacity);

        if (buf == NULL) {
            free(array);
            return 1;
        }

        array->buf = buf;
    }

    return 0;
}


void array_print(array_t* array) {
    printf("-------------------------\n");
    printf("Length: %d\n", array->length);
    printf("Capacity: %d\n", array->capacity);

    if (array->length == 0) {
        printf("Array is empty\n");
        printf("\n");
        return;
    }

    for (int i = 0; i < array->length; i++) {
        printf("index: %d, value: %d\n", i, array->buf[i]);
    }
}
