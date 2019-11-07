#pragma once

// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#include "kaitai/kaitaistruct.h"

#include <stdint.h>
#include <memory>

#if KAITAI_STRUCT_VERSION < 7000L
#error "Incompatible Kaitai Struct C++/STL API: version 0.7 or later is required"
#endif

class bcd_user_type_le_t : public kaitai::kstruct {

public:
    class ltr_obj_t;
    class rtl_obj_t;
    class leading_zero_ltr_obj_t;

    bcd_user_type_le_t(kaitai::kstream* p__io, kaitai::kstruct* p__parent = nullptr, bcd_user_type_le_t* p__root = nullptr);

private:
    void _read();

public:
    ~bcd_user_type_le_t();

    class ltr_obj_t : public kaitai::kstruct {

    public:

        ltr_obj_t(kaitai::kstream* p__io, bcd_user_type_le_t* p__parent = nullptr, bcd_user_type_le_t* p__root = nullptr);

    private:
        void _read();

    public:
        ~ltr_obj_t();

    private:
        bool f_as_int;
        int32_t m_as_int;

    public:
        int32_t as_int();

    private:
        bool f_digit2;
        int32_t m_digit2;

    public:
        int32_t digit2();

    private:
        bool f_digit4;
        int32_t m_digit4;

    public:
        int32_t digit4();

    private:
        bool f_digit3;
        int32_t m_digit3;

    public:
        int32_t digit3();

    private:
        bool f_digit5;
        int32_t m_digit5;

    public:
        int32_t digit5();

    private:
        bool f_digit8;
        int32_t m_digit8;

    public:
        int32_t digit8();

    private:
        bool f_digit6;
        int32_t m_digit6;

    public:
        int32_t digit6();

    private:
        bool f_as_str;
        std::string m_as_str;

    public:
        std::string as_str();

    private:
        bool f_digit1;
        int32_t m_digit1;

    public:
        int32_t digit1();

    private:
        bool f_digit7;
        int32_t m_digit7;

    public:
        int32_t digit7();

    private:
        uint8_t m_b1;
        uint8_t m_b2;
        uint8_t m_b3;
        uint8_t m_b4;
        bcd_user_type_le_t* m__root;
        bcd_user_type_le_t* m__parent;

    public:
        uint8_t b1() const { return m_b1; }
        uint8_t b2() const { return m_b2; }
        uint8_t b3() const { return m_b3; }
        uint8_t b4() const { return m_b4; }
        bcd_user_type_le_t* _root() const { return m__root; }
        bcd_user_type_le_t* _parent() const { return m__parent; }
    };

    class rtl_obj_t : public kaitai::kstruct {

    public:

        rtl_obj_t(kaitai::kstream* p__io, bcd_user_type_le_t* p__parent = nullptr, bcd_user_type_le_t* p__root = nullptr);

    private:
        void _read();

    public:
        ~rtl_obj_t();

    private:
        bool f_as_int;
        int32_t m_as_int;

    public:
        int32_t as_int();

    private:
        bool f_digit2;
        int32_t m_digit2;

    public:
        int32_t digit2();

    private:
        bool f_digit4;
        int32_t m_digit4;

    public:
        int32_t digit4();

    private:
        bool f_digit3;
        int32_t m_digit3;

    public:
        int32_t digit3();

    private:
        bool f_digit5;
        int32_t m_digit5;

    public:
        int32_t digit5();

    private:
        bool f_digit8;
        int32_t m_digit8;

    public:
        int32_t digit8();

    private:
        bool f_digit6;
        int32_t m_digit6;

    public:
        int32_t digit6();

    private:
        bool f_as_str;
        std::string m_as_str;

    public:
        std::string as_str();

    private:
        bool f_digit1;
        int32_t m_digit1;

    public:
        int32_t digit1();

    private:
        bool f_digit7;
        int32_t m_digit7;

    public:
        int32_t digit7();

    private:
        uint8_t m_b1;
        uint8_t m_b2;
        uint8_t m_b3;
        uint8_t m_b4;
        bcd_user_type_le_t* m__root;
        bcd_user_type_le_t* m__parent;

    public:
        uint8_t b1() const { return m_b1; }
        uint8_t b2() const { return m_b2; }
        uint8_t b3() const { return m_b3; }
        uint8_t b4() const { return m_b4; }
        bcd_user_type_le_t* _root() const { return m__root; }
        bcd_user_type_le_t* _parent() const { return m__parent; }
    };

    class leading_zero_ltr_obj_t : public kaitai::kstruct {

    public:

        leading_zero_ltr_obj_t(kaitai::kstream* p__io, bcd_user_type_le_t* p__parent = nullptr, bcd_user_type_le_t* p__root = nullptr);

    private:
        void _read();

    public:
        ~leading_zero_ltr_obj_t();

    private:
        bool f_as_int;
        int32_t m_as_int;

    public:
        int32_t as_int();

    private:
        bool f_digit2;
        int32_t m_digit2;

    public:
        int32_t digit2();

    private:
        bool f_digit4;
        int32_t m_digit4;

    public:
        int32_t digit4();

    private:
        bool f_digit3;
        int32_t m_digit3;

    public:
        int32_t digit3();

    private:
        bool f_digit5;
        int32_t m_digit5;

    public:
        int32_t digit5();

    private:
        bool f_digit8;
        int32_t m_digit8;

    public:
        int32_t digit8();

    private:
        bool f_digit6;
        int32_t m_digit6;

    public:
        int32_t digit6();

    private:
        bool f_as_str;
        std::string m_as_str;

    public:
        std::string as_str();

    private:
        bool f_digit1;
        int32_t m_digit1;

    public:
        int32_t digit1();

    private:
        bool f_digit7;
        int32_t m_digit7;

    public:
        int32_t digit7();

    private:
        uint8_t m_b1;
        uint8_t m_b2;
        uint8_t m_b3;
        uint8_t m_b4;
        bcd_user_type_le_t* m__root;
        bcd_user_type_le_t* m__parent;

    public:
        uint8_t b1() const { return m_b1; }
        uint8_t b2() const { return m_b2; }
        uint8_t b3() const { return m_b3; }
        uint8_t b4() const { return m_b4; }
        bcd_user_type_le_t* _root() const { return m__root; }
        bcd_user_type_le_t* _parent() const { return m__parent; }
    };

private:
    std::unique_ptr<ltr_obj_t> m_ltr;
    std::unique_ptr<rtl_obj_t> m_rtl;
    std::unique_ptr<leading_zero_ltr_obj_t> m_leading_zero_ltr;
    bcd_user_type_le_t* m__root;
    kaitai::kstruct* m__parent;
    std::string m__raw_ltr;
    kaitai::kstream* m__io__raw_ltr;
    std::string m__raw_rtl;
    kaitai::kstream* m__io__raw_rtl;
    std::string m__raw_leading_zero_ltr;
    kaitai::kstream* m__io__raw_leading_zero_ltr;

public:
    ltr_obj_t* ltr() const { return m_ltr.get(); }
    rtl_obj_t* rtl() const { return m_rtl.get(); }
    leading_zero_ltr_obj_t* leading_zero_ltr() const { return m_leading_zero_ltr.get(); }
    bcd_user_type_le_t* _root() const { return m__root; }
    kaitai::kstruct* _parent() const { return m__parent; }
    std::string _raw_ltr() const { return m__raw_ltr; }
    kaitai::kstream* _io__raw_ltr() const { return m__io__raw_ltr; }
    std::string _raw_rtl() const { return m__raw_rtl; }
    kaitai::kstream* _io__raw_rtl() const { return m__io__raw_rtl; }
    std::string _raw_leading_zero_ltr() const { return m__raw_leading_zero_ltr; }
    kaitai::kstream* _io__raw_leading_zero_ltr() const { return m__io__raw_leading_zero_ltr; }
};
