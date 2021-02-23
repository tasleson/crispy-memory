#include <stdlib.h>
#include <stdio.h>
#include "somelibname.h"
#include <string.h>

// File: main.c
//
// Sample library usage.
int main(void) {

    Error *e = NULL;
    int result = 0;
   
    char *s = NULL;
    result = get_some_cstr(&s);
    if (0 == result ){
        free(s);
        s = NULL;
    } else {
        printf("get_some_cstr Result = %d\n", result);
        return 10;
    }

    result = get_some_cstr_2(&s);
    if (0 == result) {
        //printf("get_some_cstr_2 returned %s\n", s);
        free(s);
        s = NULL;
    } else {
        printf("get_some_cstr_2 Result = %d\n", result);
        return 10;
    }

    e = error_new();
    const char *msg = error_msg_get(e);
    if (msg) {
        printf("error message = %s\n", msg);
        printf("error code = %d\n", error_code_get(e));
    } else {
        printf("Error msg is null :-/\n");
        return 1;
    }

    error_free(e);

    e = NULL;
    result = error_create_with_result(&e);
    if (result == 0) {
        printf("error message = %s\n", error_msg_get(e));
        printf("error code = %d\n", error_code_get(e));

        printf("Result of freeing %d\n", error_free_with_result(&e));
        printf("Value of e = %p (expecting nil)\n", e);
    } else {
        printf("Error: error_create_with_result = %d\n", result);
    }

    return 0;
}