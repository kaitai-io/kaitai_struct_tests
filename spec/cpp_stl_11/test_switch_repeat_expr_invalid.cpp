#include <boost/test/unit_test.hpp>
#include "switch_repeat_expr_invalid.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_switch_repeat_expr_invalid) {
    std::ifstream ifs("src/switch_integers.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    switch_repeat_expr_invalid_t* r = new switch_repeat_expr_invalid_t(&ks);
    BOOST_CHECK_EQUAL(r->codes()->size(), 3);
    BOOST_CHECK_EQUAL(r->codes()->at(0), 1);
    BOOST_CHECK_EQUAL(r->codes()->at(1), 7);
    BOOST_CHECK_EQUAL(r->codes()->at(2), 2);
    BOOST_CHECK_EQUAL(static_cast<switch_repeat_expr_invalid_t::one_t*>(r->body()->at(0).get())->first(), std::string("\x40\x40\x04\x37", 4));
    BOOST_CHECK_EQUAL(r->body()->at(1).get(), nullptr); // added manually
    BOOST_CHECK_EQUAL(r->_raw_body()->at(1), std::string("\x13\x00\x00\x08", 4));
    BOOST_CHECK_EQUAL(static_cast<switch_repeat_expr_invalid_t::two_t*>(r->body()->at(2).get())->second(), std::string("\x37\x13\x00\x00", 4));

    delete r;
}
