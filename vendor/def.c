#include "def.h"

// #include <stdio.h>

int gcd(int i, char c, Pair* ps) {
    int gcd;
    for (i = 1; i <= ps->n && i <= ps->m; ++i) {
        // Checks if i is factor of both integers
        if (ps->n % i == 0 && ps->m % i == 0)
            gcd = i;
    }

    return gcd;
}

// int main() {
//     Pair ps;
//     printf("Enter two integers: ");
//     scanf("%d %d", &ps.n, &ps.m);
//     printf("G.C.D of %d and %d is %d.", ps.n, ps.m, gcd(0, 'a', &ps));
// }