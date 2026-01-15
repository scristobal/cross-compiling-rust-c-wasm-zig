/**
 * @file gcd.h
 * @brief This is an example header file providing functionality XYZ.
 * @author John Doe
 * @date January 1, 2024
 */

/**
 * @brief This structure represents a pair of integers
 */
typedef struct Pair {
    int n;
    int m;
} Pair;

/**
 * @brief This function performs the greatest common division (gcd) of a `Pair`
 *
 * @param i Minimum gcd to search for.
 * @param ps Pair of integers to find the gcd.
 * @return Greatest common divisor of `ps` that is larger or equal to `i`.
 */
int gcd(int i, Pair* ps);
