#include "gcd.h"

int gcd(Pair* ps) {
    int i = 1;
    int gcd;
    for (; i <= ps->n && i <= ps->m; ++i) {
        // Checks if i is factor of both integers
        if (ps->n % i == 0 && ps->m % i == 0)
            gcd = i;
    }

    return gcd;
}
