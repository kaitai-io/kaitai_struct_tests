#include <boost/test/unit_test.hpp>

#include <switch_bytearray.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_switch_bytearray) {
    std::ifstream ifs("src/switch_opcodes.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    switch_bytearray_t* r = new switch_bytearray_t(&ks);

    BOOST_CHECK_EQUAL(r->opcodes()->size(), 4);

    BOOST_CHECK_EQUAL(r->opcodes()->at(0)->code(), "S");
    BOOST_CHECK_EQUAL((static_cast<switch_bytearray_t::opcode_t::strval_t*>(r->opcodes()->at(0)->body()))->value(), "foobar");

    BOOST_CHECK_EQUAL(r->opcodes()->at(1)->code(), "I");
    BOOST_CHECK_EQUAL((static_cast<switch_bytearray_t::opcode_t::intval_t*>(r->opcodes()->at(1)->body()))->value(), 0x42);

    BOOST_CHECK_EQUAL(r->opcodes()->at(2)->code(), "I");
    BOOST_CHECK_EQUAL((static_cast<switch_bytearray_t::opcode_t::intval_t*>(r->opcodes()->at(2)->body()))->value(), 0x37);

    BOOST_CHECK_EQUAL(r->opcodes()->at(3)->code(), "S");
    BOOST_CHECK_EQUAL((static_cast<switch_bytearray_t::opcode_t::strval_t*>(r->opcodes()->at(3)->body()))->value(), "");

    delete r;
}
