#include "./array.c"
#include <assert.h>
#include <stdio.h>


int test_array_insert(array_t* array) {
    printf("test_array_insert: ");

    for (int i = 0; i < 20; i++) {
        if (array_insert(array, i, i) != 0) {
            return 1;
        }
    }

    assert(array->length == 20);
    assert(array->buf[0] == 0);
    assert(array->buf[array->length-1] == 19);

    printf("passed\n");
    return 0;
}


int test_array_delete(array_t* array) {
    printf("test_array_delete: ");

    for (int i = 0; i < 20; i++) {
        if (array_delete(array, 0) != 0) {
            return 1;
        }
    }

    assert(array->length == 0);

    printf("passed\n");
    return 0;
}


int main() {
    array_t* array = array_new(2);

    if (array == NULL) {
        return 1;
    }

    assert(test_array_insert(array) == 0);
    assert(test_array_delete(array) == 0);

    return 0;
}
