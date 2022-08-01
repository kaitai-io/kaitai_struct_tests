#include "custom_fx_no_args.h"

custom_fx_no_args_t::custom_fx_no_args_t() {
}

std::string custom_fx_no_args_t::decode(std::string src) {
    return "_" + src + "_";
}
