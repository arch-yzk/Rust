#include <stdlib.h>

int main()
{
    int *ptr = malloc(sizeof(int));
    //メモリ領域を確保
    free(ptr);
    *ptr = 5;
}