#include <stdio.h>

#include "def.h"

int main() {
    Pair ps;
    printf("Enter two integers: ");
    scanf("%d %d", &ps.n, &ps.m);
    printf("G.C.D of %d and %d is %d.", ps.n, ps.m, gcd(0, &ps));
}