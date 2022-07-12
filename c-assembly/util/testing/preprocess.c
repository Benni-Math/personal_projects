#include <stdio.h>
#include <math.h>

#define Peval(cmd) printf(#cmd ": %g\n", cmd)

#define Setup_list(name, ...)                               \
    double *name ## _list = (double []){__VA_ARGS__, NAN};  \
    int name ## _len = 0;                                   \
    for (name ## _len = 0;                                  \
            !isnan(name ## _list[name ## _len]);            \
            ) name ## _len ++;

int main() {
    double *plist = (double[]){1, 2, 3};
    double list[] = {1, 2, 3};

    printf("Using a macro to check sizeof:\n\n");

    Peval(sizeof(plist)/(sizeof(double)+0.0));
    Peval(sizeof(list)/(sizeof(double)+0.0));

    printf("\nUsing a macro to construct lists with a len:\n\n");

    Setup_list(items, 1, 2, 4, 8);
    double sum = 0;

    for (double *ptr = items_list; !isnan(*ptr); ptr++)
        sum += *ptr;
    printf("Total sum of items in list: %g\n", sum);

    #define Length(in) in ## _len

    sum = 0;
    Setup_list(next_set, -1, 2.2, 4.8, 0.1);
    for (int i=0; i < Length(next_set); i++)
        sum += next_set_list[i];
    printf("Total for next set list: %g\n", sum);
}

