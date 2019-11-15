#include "my_custom_fx.h"

my_custom_fx_t::my_custom_fx_t(int p_key, bool p_flag, std::string p_some_bytes) {
    key = p_flag ? p_key : -p_key;
}

std::string my_custom_fx_t::decode(std::string src) {
    int len = src.length();
    std::string dst(len, ' ');

    for (int i = 0; i < len; i++)
        dst[i] = src[i] + key;

    return dst;
}
