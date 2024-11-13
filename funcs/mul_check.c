#include <stdio.h>
#include <limits.h>

int i32_is_mul_overflow(int x, int y){
    int p = x * y;
    return x && p/x!=y;  // true means overflow
}

int i64_is_mul_overflow(long long x, long long y){
    long long p = x * y;
    return x && p/x!=y;  // true means overflow
}

int main(void){
    i32_is_mul_overflow(INT_MAX,2);
    i64_is_mul_overflow(LLONG_MAX,2);
}