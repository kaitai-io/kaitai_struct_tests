#include <boost/test/unit_test.hpp>

#include <switch_manual_enum.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_switch_manual_enum) {
    std::ifstream ifs("src/switch_opcodes.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    switch_manual_enum_t* r = new switch_manual_enum_t(&ks);

    BOOST_CHECK_EQUAL(r->opcodes()->size(), 4);

    BOOST_CHECK_EQUAL(r->opcodes()->at(0)->code(), switch_manual_enum_t::opcode_t::CODE_ENUM_STRVAL);
    BOOST_CHECK_EQUAL((static_cast<switch_manual_enum_t::opcode_t::strval_t*>(r->opcodes()->at(0)->body()))->value(), "foobar");

    BOOST_CHECK_EQUAL(r->opcodes()->at(1)->code(), switch_manual_enum_t::opcode_t::CODE_ENUM_INTVAL);
    BOOST_CHECK_EQUAL((static_cast<switch_manual_enum_t::opcode_t::intval_t*>(r->opcodes()->at(1)->body()))->value(), 0x42);

    BOOST_CHECK_EQUAL(r->opcodes()->at(2)->code(), switch_manual_enum_t::opcode_t::CODE_ENUM_INTVAL);
    BOOST_CHECK_EQUAL((static_cast<switch_manual_enum_t::opcode_t::intval_t*>(r->opcodes()->at(2)->body()))->value(), 0x37);

    BOOST_CHECK_EQUAL(r->opcodes()->at(3)->code(), switch_manual_enum_t::opcode_t::CODE_ENUM_STRVAL);
    BOOST_CHECK_EQUAL((static_cast<switch_manual_enum_t::opcode_t::strval_t*>(r->opcodes()->at(3)->body()))->value(), "");

    delete r;
}
