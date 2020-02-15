#ifndef CUSTOM_FX_NO_ARGS_H_
#define CUSTOM_FX_NO_ARGS_H_

#include <kaitai/custom_decoder.h>

class custom_fx_no_args_t : public kaitai::custom_decoder {
public:
    custom_fx_no_args_t();
    std::string decode(std::string src);
};

#endif  // CUSTOM_FX_NO_ARGS_H_
