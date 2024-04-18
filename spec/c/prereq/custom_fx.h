#ifndef CUSTOM_FX_H_
#define CUSTOM_FX_H_

#include <kaitaistruct.h>

ks_custom_decoder custom_fx_create(int p_key);
void custom_fx_destroy(ks_custom_decoder decoder);

#endif
