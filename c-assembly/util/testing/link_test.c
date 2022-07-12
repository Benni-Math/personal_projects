// This file is just meant for testing certain issues I'm having with the
// linker

#include <curl/curl.h>
#include <stdio.h>

#include <strings.h>
#include <stdlib.h>

typedef struct keyval {
    char *key;
    void *value;
} keyval;

typedef struct dictionary {
    keyval **pairs;
    int length;
} dictionary;
    

int main() {
    // First, we're testing that we can dynamically link curl
    CURL *curl;
    CURLcode res;

    curl = curl_easy_init();

    if (curl) {
        curl_easy_setopt(curl, CURLOPT_URL, "http://google.com");
        curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L);
        res = curl_easy_perform(curl);

        if (res != CURLE_OK)
            fprintf(stderr, "curl_easy_perform() failed: %s\n",
                curl_easy_strerror(res));

        curl_easy_cleanup(curl);
    }

    // Now, I'm testing for an issue that I'm having with ../void/dict.c
    printf("\n\nTesting strcasecmp: %d\n", strcasecmp("hello", "hello"));

    // Testing that malloc is ok
    dictionary *d = malloc(sizeof(dictionary));
    *d = (dictionary){ };

    keyval *kv = malloc(sizeof(keyval));
    *kv = (keyval){.key = "test", .value = "test-val"};

    d->length++;
    d->pairs = realloc(d->pairs, (sizeof(keyval*)) * (d->length));
    d->pairs[d->length - 1] = kv;

    
    printf("\nThe key-value pair is: %s->%s\n",
        (char *)d->pairs[0]->key,
        (char *)d->pairs[0]->value);

    printf("\nNow let's try freeing the variables!\n");

    free(kv);
    free(d);
    
    printf("\nSuccessfully freed the (mock) dictionary.\n");

    char two[] = "two";
    printf("Testing character array: %s\n", two);
}

