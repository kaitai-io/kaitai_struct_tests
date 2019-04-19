// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#include <memory>
#include "bcd_user_type_be.h"

#include <memory>

bcd_user_type_be_t::bcd_user_type_be_t(kaitai::kstream* p__io, kaitai::kstruct* p__parent, bcd_user_type_be_t* p__root) : kaitai::kstruct(p__io) {
    m__parent = p__parent;
    m__root = this;
    m_ltr = nullptr;
    m__io__raw_ltr = nullptr;
    m_rtl = nullptr;
    m__io__raw_rtl = nullptr;
    m_leading_zero_ltr = nullptr;
    m__io__raw_leading_zero_ltr = nullptr;
    _read();
}

void bcd_user_type_be_t::_read() {
    m__raw_ltr = m__io->read_bytes(4);
    m__io__raw_ltr = new kaitai::kstream(m__raw_ltr);
    m_ltr = std::unique_ptr<ltr_obj_t>(new ltr_obj_t(m__io__raw_ltr, this, m__root));
    m__raw_rtl = m__io->read_bytes(4);
    m__io__raw_rtl = new kaitai::kstream(m__raw_rtl);
    m_rtl = std::unique_ptr<rtl_obj_t>(new rtl_obj_t(m__io__raw_rtl, this, m__root));
    m__raw_leading_zero_ltr = m__io->read_bytes(4);
    m__io__raw_leading_zero_ltr = new kaitai::kstream(m__raw_leading_zero_ltr);
    m_leading_zero_ltr = std::unique_ptr<leading_zero_ltr_obj_t>(new leading_zero_ltr_obj_t(m__io__raw_leading_zero_ltr, this, m__root));
}

bcd_user_type_be_t::~bcd_user_type_be_t() {
    delete m__io__raw_ltr;
    delete m__io__raw_rtl;
    delete m__io__raw_leading_zero_ltr;
}

bcd_user_type_be_t::ltr_obj_t::ltr_obj_t(kaitai::kstream* p__io, bcd_user_type_be_t* p__parent, bcd_user_type_be_t* p__root) : kaitai::kstruct(p__io) {
    m__parent = p__parent;
    m__root = p__root;
    f_as_int = false;
    f_digit2 = false;
    f_digit4 = false;
    f_digit3 = false;
    f_digit5 = false;
    f_digit8 = false;
    f_digit6 = false;
    f_as_str = false;
    f_digit1 = false;
    f_digit7 = false;
    _read();
}

void bcd_user_type_be_t::ltr_obj_t::_read() {
    m_b1 = m__io->read_u1();
    m_b2 = m__io->read_u1();
    m_b3 = m__io->read_u1();
    m_b4 = m__io->read_u1();
}

bcd_user_type_be_t::ltr_obj_t::~ltr_obj_t() {
}

int32_t bcd_user_type_be_t::ltr_obj_t::as_int() {
    if (f_as_int)
        return m_as_int;
    m_as_int = ((((((((digit8() * 1) + (digit7() * 10)) + (digit6() * 100)) + (digit5() * 1000)) + (digit4() * 10000)) + (digit3() * 100000)) + (digit2() * 1000000)) + (digit1() * 10000000));
    f_as_int = true;
    return m_as_int;
}

int32_t bcd_user_type_be_t::ltr_obj_t::digit2() {
    if (f_digit2)
        return m_digit2;
    m_digit2 = (b1() & 15);
    f_digit2 = true;
    return m_digit2;
}

int32_t bcd_user_type_be_t::ltr_obj_t::digit4() {
    if (f_digit4)
        return m_digit4;
    m_digit4 = (b2() & 15);
    f_digit4 = true;
    return m_digit4;
}

int32_t bcd_user_type_be_t::ltr_obj_t::digit3() {
    if (f_digit3)
        return m_digit3;
    m_digit3 = ((b2() & 240) >> 4);
    f_digit3 = true;
    return m_digit3;
}

int32_t bcd_user_type_be_t::ltr_obj_t::digit5() {
    if (f_digit5)
        return m_digit5;
    m_digit5 = ((b3() & 240) >> 4);
    f_digit5 = true;
    return m_digit5;
}

int32_t bcd_user_type_be_t::ltr_obj_t::digit8() {
    if (f_digit8)
        return m_digit8;
    m_digit8 = (b4() & 15);
    f_digit8 = true;
    return m_digit8;
}

int32_t bcd_user_type_be_t::ltr_obj_t::digit6() {
    if (f_digit6)
        return m_digit6;
    m_digit6 = (b3() & 15);
    f_digit6 = true;
    return m_digit6;
}

std::string bcd_user_type_be_t::ltr_obj_t::as_str() {
    if (f_as_str)
        return m_as_str;
    m_as_str = kaitai::kstream::to_string(digit1()) + kaitai::kstream::to_string(digit2()) + kaitai::kstream::to_string(digit3()) + kaitai::kstream::to_string(digit4()) + kaitai::kstream::to_string(digit5()) + kaitai::kstream::to_string(digit6()) + kaitai::kstream::to_string(digit7()) + kaitai::kstream::to_string(digit8());
    f_as_str = true;
    return m_as_str;
}

int32_t bcd_user_type_be_t::ltr_obj_t::digit1() {
    if (f_digit1)
        return m_digit1;
    m_digit1 = ((b1() & 240) >> 4);
    f_digit1 = true;
    return m_digit1;
}

int32_t bcd_user_type_be_t::ltr_obj_t::digit7() {
    if (f_digit7)
        return m_digit7;
    m_digit7 = ((b4() & 240) >> 4);
    f_digit7 = true;
    return m_digit7;
}

bcd_user_type_be_t::rtl_obj_t::rtl_obj_t(kaitai::kstream* p__io, bcd_user_type_be_t* p__parent, bcd_user_type_be_t* p__root) : kaitai::kstruct(p__io) {
    m__parent = p__parent;
    m__root = p__root;
    f_as_int = false;
    f_digit2 = false;
    f_digit4 = false;
    f_digit3 = false;
    f_digit5 = false;
    f_digit8 = false;
    f_digit6 = false;
    f_as_str = false;
    f_digit1 = false;
    f_digit7 = false;
    _read();
}

void bcd_user_type_be_t::rtl_obj_t::_read() {
    m_b1 = m__io->read_u1();
    m_b2 = m__io->read_u1();
    m_b3 = m__io->read_u1();
    m_b4 = m__io->read_u1();
}

bcd_user_type_be_t::rtl_obj_t::~rtl_obj_t() {
}

int32_t bcd_user_type_be_t::rtl_obj_t::as_int() {
    if (f_as_int)
        return m_as_int;
    m_as_int = ((((((((digit1() * 1) + (digit2() * 10)) + (digit3() * 100)) + (digit4() * 1000)) + (digit5() * 10000)) + (digit6() * 100000)) + (digit7() * 1000000)) + (digit8() * 10000000));
    f_as_int = true;
    return m_as_int;
}

int32_t bcd_user_type_be_t::rtl_obj_t::digit2() {
    if (f_digit2)
        return m_digit2;
    m_digit2 = (b1() & 15);
    f_digit2 = true;
    return m_digit2;
}

int32_t bcd_user_type_be_t::rtl_obj_t::digit4() {
    if (f_digit4)
        return m_digit4;
    m_digit4 = (b2() & 15);
    f_digit4 = true;
    return m_digit4;
}

int32_t bcd_user_type_be_t::rtl_obj_t::digit3() {
    if (f_digit3)
        return m_digit3;
    m_digit3 = ((b2() & 240) >> 4);
    f_digit3 = true;
    return m_digit3;
}

int32_t bcd_user_type_be_t::rtl_obj_t::digit5() {
    if (f_digit5)
        return m_digit5;
    m_digit5 = ((b3() & 240) >> 4);
    f_digit5 = true;
    return m_digit5;
}

int32_t bcd_user_type_be_t::rtl_obj_t::digit8() {
    if (f_digit8)
        return m_digit8;
    m_digit8 = (b4() & 15);
    f_digit8 = true;
    return m_digit8;
}

int32_t bcd_user_type_be_t::rtl_obj_t::digit6() {
    if (f_digit6)
        return m_digit6;
    m_digit6 = (b3() & 15);
    f_digit6 = true;
    return m_digit6;
}

std::string bcd_user_type_be_t::rtl_obj_t::as_str() {
    if (f_as_str)
        return m_as_str;
    m_as_str = kaitai::kstream::to_string(digit8()) + kaitai::kstream::to_string(digit7()) + kaitai::kstream::to_string(digit6()) + kaitai::kstream::to_string(digit5()) + kaitai::kstream::to_string(digit4()) + kaitai::kstream::to_string(digit3()) + kaitai::kstream::to_string(digit2()) + kaitai::kstream::to_string(digit1());
    f_as_str = true;
    return m_as_str;
}

int32_t bcd_user_type_be_t::rtl_obj_t::digit1() {
    if (f_digit1)
        return m_digit1;
    m_digit1 = ((b1() & 240) >> 4);
    f_digit1 = true;
    return m_digit1;
}

int32_t bcd_user_type_be_t::rtl_obj_t::digit7() {
    if (f_digit7)
        return m_digit7;
    m_digit7 = ((b4() & 240) >> 4);
    f_digit7 = true;
    return m_digit7;
}

bcd_user_type_be_t::leading_zero_ltr_obj_t::leading_zero_ltr_obj_t(kaitai::kstream* p__io, bcd_user_type_be_t* p__parent, bcd_user_type_be_t* p__root) : kaitai::kstruct(p__io) {
    m__parent = p__parent;
    m__root = p__root;
    f_as_int = false;
    f_digit2 = false;
    f_digit4 = false;
    f_digit3 = false;
    f_digit5 = false;
    f_digit8 = false;
    f_digit6 = false;
    f_as_str = false;
    f_digit1 = false;
    f_digit7 = false;
    _read();
}

void bcd_user_type_be_t::leading_zero_ltr_obj_t::_read() {
    m_b1 = m__io->read_u1();
    m_b2 = m__io->read_u1();
    m_b3 = m__io->read_u1();
    m_b4 = m__io->read_u1();
}

bcd_user_type_be_t::leading_zero_ltr_obj_t::~leading_zero_ltr_obj_t() {
}

int32_t bcd_user_type_be_t::leading_zero_ltr_obj_t::as_int() {
    if (f_as_int)
        return m_as_int;
    m_as_int = ((((((((digit8() * 1) + (digit7() * 10)) + (digit6() * 100)) + (digit5() * 1000)) + (digit4() * 10000)) + (digit3() * 100000)) + (digit2() * 1000000)) + (digit1() * 10000000));
    f_as_int = true;
    return m_as_int;
}

int32_t bcd_user_type_be_t::leading_zero_ltr_obj_t::digit2() {
    if (f_digit2)
        return m_digit2;
    m_digit2 = (b1() & 15);
    f_digit2 = true;
    return m_digit2;
}

int32_t bcd_user_type_be_t::leading_zero_ltr_obj_t::digit4() {
    if (f_digit4)
        return m_digit4;
    m_digit4 = (b2() & 15);
    f_digit4 = true;
    return m_digit4;
}

int32_t bcd_user_type_be_t::leading_zero_ltr_obj_t::digit3() {
    if (f_digit3)
        return m_digit3;
    m_digit3 = ((b2() & 240) >> 4);
    f_digit3 = true;
    return m_digit3;
}

int32_t bcd_user_type_be_t::leading_zero_ltr_obj_t::digit5() {
    if (f_digit5)
        return m_digit5;
    m_digit5 = ((b3() & 240) >> 4);
    f_digit5 = true;
    return m_digit5;
}

int32_t bcd_user_type_be_t::leading_zero_ltr_obj_t::digit8() {
    if (f_digit8)
        return m_digit8;
    m_digit8 = (b4() & 15);
    f_digit8 = true;
    return m_digit8;
}

int32_t bcd_user_type_be_t::leading_zero_ltr_obj_t::digit6() {
    if (f_digit6)
        return m_digit6;
    m_digit6 = (b3() & 15);
    f_digit6 = true;
    return m_digit6;
}

std::string bcd_user_type_be_t::leading_zero_ltr_obj_t::as_str() {
    if (f_as_str)
        return m_as_str;
    m_as_str = kaitai::kstream::to_string(digit1()) + kaitai::kstream::to_string(digit2()) + kaitai::kstream::to_string(digit3()) + kaitai::kstream::to_string(digit4()) + kaitai::kstream::to_string(digit5()) + kaitai::kstream::to_string(digit6()) + kaitai::kstream::to_string(digit7()) + kaitai::kstream::to_string(digit8());
    f_as_str = true;
    return m_as_str;
}

int32_t bcd_user_type_be_t::leading_zero_ltr_obj_t::digit1() {
    if (f_digit1)
        return m_digit1;
    m_digit1 = ((b1() & 240) >> 4);
    f_digit1 = true;
    return m_digit1;
}

int32_t bcd_user_type_be_t::leading_zero_ltr_obj_t::digit7() {
    if (f_digit7)
        return m_digit7;
    m_digit7 = ((b4() & 240) >> 4);
    f_digit7 = true;
    return m_digit7;
}
