#include <cstdio>
char* get(){
    char data[16] = "hello world";
    return data;
}

int main(){
    printf("hello");
    char* p = "gello world";
    p[0] = 'h';
    printf("%s",p);
    printf("end");
}