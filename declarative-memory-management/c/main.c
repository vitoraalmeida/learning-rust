#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BUF_SIZE 256 * 1024
#define MAX_RESULTS 128

struct Mistake {
    // human-readable description of the mistake
    char *message;

    // points to where the mistake is in the text
    char *location;
};

struct CheckResult {
    // name of the file we checked
    char *path;

    // NULL if manuscript was fine, otherwise
    // describes what's wrong with it
    struct Mistake *mistake;
};

struct CheckResult check(char *path, char *buf) {
    struct CheckResult result;
    result.path = path;
    result.mistake = NULL;

    FILE *f = fopen(path, "r");
    size_t len = fread(buf, 1, BUF_SIZE - 1, f);
    fclose(f);
    buf[len] = '\0';

    for (size_t i = 0; i < len; i++) {
        if (buf[i] == ',') {
            struct Mistake *m = malloc(sizeof(struct Mistake));

            size_t location_len = len - i;
            if (location_len > 12) {
                location_len = 12;
            }

            m->location = malloc(location_len + 1);
            memcpy(m->location, &buf[i], location_len);
            m->location[location_len] = '\0';

            m->message = "commas are forbiden";
            result.mistake = m;
            break;
        }
    }

    return result;
}

void report(struct CheckResult result) {
    printf("\n");
    printf("~ %s ~\n", result.path);
    if (result.mistake == NULL) {
        printf("no mistakes!\n");
    } else {
        printf("mistake: %s: '%.12s'\n", 
                result.mistake->message,
                result.mistake->location
        );
    }
}

int main (int argc, char **argv) {
    char buf[BUF_SIZE];

    struct CheckResult results[MAX_RESULTS];
    int num_results = 0;

    for (int i = 1; i < argc; i++) {
        char *path = argv[i];
        struct CheckResult result = check(path, buf);
        results[num_results++] = result;
    }

    for (int i = 0; i < num_results; i++) {
        report(results[i]);
    }

    for (int i = 0; i < num_results; i++) {
        if (results[i].mistake == NULL) {
            continue;
        }
        free(results[i].mistake->location);
        free(results[i].mistake);
    }

    return 0;
}
