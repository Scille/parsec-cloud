#include "Enclave.h"
#include "Enclave_t.h"  /* print_string */

/*
 * printf:
 *   Invokes OCALL to display the enclave buffer to the terminal.
 */
void printf(const char* fmt, ...)
{
    char buf[BUFSIZ] = { '\0' };
    va_list ap;
    va_start(ap, fmt);
    vsnprintf(buf, BUFSIZ, fmt, ap);
    va_end(ap);
    ocall_print_string(buf);
}

char* cutoff(const char* str, int from, int to)
{
    if (from >= to)
        return  NULL;

    char* cut = (char*)calloc(sizeof(char), (to - from) + 1);
    char* begin = cut;
    if (!cut)
        return  NULL;

    const char* fromit = str + from;
    const char* toit = str + to;
    (void)toit;
    memcpy(cut, fromit, to);
    return begin;
}

int get_ul_len(unsigned long value) {
    int l = 1;
    while (value > 9) { l++; value /= 10; }
    return l;
}

void strtolower(char* src) {
    for (unsigned int i = 0; i < strlen(src); i++)
    {
        if (src[i] >= 'A' && src[i] <= 'Z')
            src[i] = src[i] + 32;
    }
}

void rot_one(char* str)
{
    strtolower(str);
    for (unsigned int i = 0; i < strlen(str); i++)
    {
        if (str[i] >= 'a' && str[i] < 'z')
            str[i] = str[i] + 1;
        else
            str[i] = 'a';
    }
}