#include <boost/test/unit_test.hpp>

#include <switch_manual_str_else.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_switch_manual_str_else) {
    std::ifstream ifs("src/switch_opcodes2.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    switch_manual_str_else_t* r = new switch_manual_str_else_t(&ks);

    BOOST_CHECK_EQUAL(r->opcodes()->size(), 4);

    BOOST_CHECK_EQUAL(r->opcodes()->at(0)->code(), "S");
    BOOST_CHECK_EQUAL((static_cast<switch_manual_str_else_t::opcode_t::strval_t*>(r->opcodes()->at(0)->body()))->value(), "foo");

    BOOST_CHECK_EQUAL(r->opcodes()->at(1)->code(), "X");
    BOOST_CHECK_EQUAL((static_cast<switch_manual_str_else_t::opcode_t::noneval_t*>(r->opcodes()->at(1)->body()))->filler(), 0x42);

    BOOST_CHECK_EQUAL(r->opcodes()->at(2)->code(), "Y");
    BOOST_CHECK_EQUAL((static_cast<switch_manual_str_else_t::opcode_t::noneval_t*>(r->opcodes()->at(2)->body()))->filler(), 0xcafe);

    BOOST_CHECK_EQUAL(r->opcodes()->at(3)->code(), "I");
    BOOST_CHECK_EQUAL((static_cast<switch_manual_str_else_t::opcode_t::intval_t*>(r->opcodes()->at(3)->body()))->value(), 7);

    delete r;
}
