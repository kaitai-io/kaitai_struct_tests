#include <boost/test/unit_test.hpp>
#include "switch_manual_enum_invalid_else.h"
#include <iostream>
#include <fstream>
#include <vector>


BOOST_AUTO_TEST_CASE(test_switch_manual_enum_invalid_else) {
    std::ifstream ifs("src/enum_negative.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    switch_manual_enum_invalid_else_t* r = new switch_manual_enum_invalid_else_t(&ks);


    BOOST_CHECK_EQUAL(r->opcodes()->size(), 2);
    BOOST_CHECK_EQUAL(r->opcodes()->at(0)->code(), static_cast<switch_manual_enum_invalid_else_t::opcode_t::code_enum_t>(255));
    BOOST_CHECK_EQUAL(static_cast<switch_manual_enum_invalid_else_t::opcode_t::defval_t*>(r->opcodes()->at(0)->body())->value(), 123);
    BOOST_CHECK_EQUAL(r->opcodes()->at(1)->code(), static_cast<switch_manual_enum_invalid_else_t::opcode_t::code_enum_t>(1));
    BOOST_CHECK_EQUAL(static_cast<switch_manual_enum_invalid_else_t::opcode_t::defval_t*>(r->opcodes()->at(1)->body())->value(), 123);

    delete r;
}
