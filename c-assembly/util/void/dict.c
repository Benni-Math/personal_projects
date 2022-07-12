// Basic dictionary implementation from "21st Century C"
//      A basic OOP example in C

#include <stdio.h>
#include <stdlib.h>
#include "dict.h"

void *dictionary_not_found; // public marker for when a key isn't found

dictionary *new_dictionary(void) {
    static int dnf;
    if (!dictionary_not_found) dictionary_not_found = &dnf;
    dictionary *out = malloc(sizeof(dictionary));
    *out = (dictionary){ };
    return out;
}

static void dictionary_add_keyval(dictionary *in, keyval *kv) {
    (in->length)++;
    // this realloc is not properly error-handled
    in->pairs = realloc(in->pairs, (sizeof(keyval*)) * (in->length));
    in->pairs[in->length - 1] = kv;
}

void dictionary_add(dictionary *in, char *key, void *value) {
    // if (!key) { fprintf(stderr, "NULL is not a valid key.\n"); abort(); }
    dictionary_add_keyval(in, keyval_new(key, value));
}

void *dictionary_find(dictionary const *in, char const *key) {
    for (int i=0; i < in->length; i++)
        if (keyval_matches(in->pairs[i], key))
            return in->pairs[i]->value;
    return dictionary_not_found;
}

dictionary *dictionary_copy(dictionary *in) {
    dictionary *out = new_dictionary();
    for (int i=0; i < in->length; i++) {
        dictionary_add_keyval(out, keyval_copy(in->pairs[i]));
    }
    return out;
}

void dictionary_free(dictionary *in) {
    for (int i=0; i < in->length; i++) {
        keyval_free(in->pairs[i]);
    }
    free(in);
}

