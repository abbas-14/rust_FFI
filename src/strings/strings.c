#include<stdio.h>
#include<string.h>
#include<stdlib.h>

char* str_rev(char* istr) {
    int len = strlen(istr) + 1, i, j=0;
    char* ostr = malloc(len * sizeof(char));
    *(ostr + len - 1) = '\0';
    for(i = len - 2; i>=0; *(ostr + i) = *(istr + j), --i, ++j);
    return ostr;
}

short str_cmp(char* istr1, char* istr2) {
    int len1 = strlen(istr1) + 1,
        len2 = strlen(istr2) + 1,
        i = 0;
    if(len1 > len2) return 1;
    if(len1 < len2) return -1;

    for(; i < len1; ++i) {
        char s = *(istr1 + i);
        char d = *(istr2 + i);
        if(s > d) return 1;
        if(s < d) return -1;
    }
    return 0;
}