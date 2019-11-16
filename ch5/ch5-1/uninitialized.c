#include <stdio.h>
#include <stdlib.h>

int main()
{
    int *ptr = malloc(sizeof(int));//メモリを確保

    printf("*ptr = %d\n", *ptr);//初期化していないメモリ領域にアクセス
    free(ptr);
}