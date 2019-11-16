#include <stdlib.h>

int main()
{
    int *ptr1 = malloc(sizeof(int));//メモリ領域を確保

    int *ptr2 = ptr1;//ポインタをコピー
    free(ptr1);
    free(ptr2);
}