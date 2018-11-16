#include <boost/test/unit_test.hpp>

#include <switch_repeat_expr.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_switch_repeat_expr) {
    std::ifstream ifs("src/switch_tlv.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    switch_repeat_expr_t* r = new switch_repeat_expr_t(&ks);

    BOOST_CHECK_EQUAL(r->code(), 0x11);
    BOOST_CHECK_EQUAL(r->size(), 9);
    BOOST_CHECK_EQUAL((static_cast<switch_repeat_expr_t::one_t*>(r->body()->at(0)))->first(), std::string("Stuff\0Me\0", 9));

    delete r;
}
