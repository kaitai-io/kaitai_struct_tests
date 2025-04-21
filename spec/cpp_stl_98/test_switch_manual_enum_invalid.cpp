#include <boost/test/unit_test.hpp>
#include "switch_manual_enum_invalid.h"
#include <iostream>
#include <fstream>
#include <vector>


BOOST_AUTO_TEST_CASE(test_switch_manual_enum_invalid) {
    std::ifstream ifs("src/enum_negative.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    switch_manual_enum_invalid_t* r = new switch_manual_enum_invalid_t(&ks);


    BOOST_CHECK_EQUAL(r->opcodes()->size(), 2);
    BOOST_CHECK_EQUAL(r->opcodes()->at(0)->code(), static_cast<switch_manual_enum_invalid_t::opcode_t::code_enum_t>(255));
    BOOST_CHECK(r->opcodes()->at(0)->_is_null_body());
    BOOST_CHECK(!r->opcodes()->at(0)->body()); // added manually
    BOOST_CHECK_EQUAL(r->opcodes()->at(1)->code(), static_cast<switch_manual_enum_invalid_t::opcode_t::code_enum_t>(1));
    BOOST_CHECK(r->opcodes()->at(1)->_is_null_body());
    BOOST_CHECK(!r->opcodes()->at(1)->body()); // added manually

    delete r;
}
