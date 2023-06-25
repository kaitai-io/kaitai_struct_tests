#ifndef MY_CUSTOM_FX_H_
#define MY_CUSTOM_FX_H_

#include <kaitaistruct.h>

ks_custom_decoder my_custom_fx_create(int p_key, int p_flag, ks_bytes* p_some_bytes);
void my_custom_fx_destroy(ks_custom_decoder decoder);

#endif
