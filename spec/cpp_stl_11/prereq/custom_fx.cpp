#include "custom_fx.h"

nested_t::deeply_t::custom_fx_t::custom_fx_t(int p_key) {
}

std::string nested_t::deeply_t::custom_fx_t::decode(std::string src) {
    return "_" + src + "_";
}
