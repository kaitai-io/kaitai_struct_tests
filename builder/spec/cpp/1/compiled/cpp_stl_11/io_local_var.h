#pragma once

// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#include "kaitai/kaitaistruct.h"

#include <stdint.h>
#include <memory>

#if KAITAI_STRUCT_VERSION < 7000L
#error "Incompatible Kaitai Struct C++/STL API: version 0.7 or later is required"
#endif

class io_local_var_t : public kaitai::kstruct {

public:
    class dummy_t;

    io_local_var_t(kaitai::kstream* p__io, kaitai::kstruct* p__parent = nullptr, io_local_var_t* p__root = nullptr, bool p__is_own_io = false);

private:
    std::unique_ptr<kaitai::kstream> m__own_io;
    void _read();

public:
    ~io_local_var_t();

    class dummy_t : public kaitai::kstruct {

    public:

        dummy_t(kaitai::kstream* p__io, io_local_var_t* p__parent = nullptr, io_local_var_t* p__root = nullptr, bool p__is_own_io = false);

    private:
        std::unique_ptr<kaitai::kstream> m__own_io;
        void _read();

    public:
        ~dummy_t();

    private:
        io_local_var_t* m__root;
        io_local_var_t* m__parent;

    public:
        io_local_var_t* _root() const { return m__root; }
        io_local_var_t* _parent() const { return m__parent; }
    };

private:
    bool f_mess_up;
    std::unique_ptr<dummy_t> m_mess_up;

public:
    dummy_t* mess_up();

private:
    std::string m_skip;
    uint8_t m_always_null;
    bool n_always_null;

public:
    bool _is_null_always_null() { always_null(); return n_always_null; };

private:
    uint8_t m_followup;
    io_local_var_t* m__root;
    kaitai::kstruct* m__parent;
    std::string m__raw_mess_up;

public:
    std::string skip() const { return m_skip; }
    uint8_t always_null() const { return m_always_null; }
    uint8_t followup() const { return m_followup; }
    io_local_var_t* _root() const { return m__root; }
    kaitai::kstruct* _parent() const { return m__parent; }
    std::string _raw_mess_up() const { return m__raw_mess_up; }
};
