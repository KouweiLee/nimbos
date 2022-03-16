#include <stddef.h>
#include <stdlib.h>
#include <unistd.h>

static uint64_t seed;

void srand(unsigned s)
{
    seed = s - 1;
}

int rand(void)
{
    seed = 6364136223846793005ULL * seed + 1;
    return seed >> 33;
}

_Noreturn void abort(void)
{
    exit(127);
}
