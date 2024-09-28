#define CRYPTOGRAF_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void init_sodium();
void poly1305_hash();
void blake2_hash();
void chacha20_encode();
void chacha20_decode();