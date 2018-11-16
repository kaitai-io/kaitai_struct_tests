#ifndef MY_CUSTOM_FX_H_
#define MY_CUSTOM_FX_H_

#include <kaitai/custom_decoder.h>

class my_custom_fx_t : public kaitai::custom_decoder {
public:
    my_custom_fx_t(int p_key, bool p_flag, std::string p_some_bytes);
    std::string decode(std::string src);
private:
    int key;
};

#endif  // MY_CUSTOM_FX_H_
