#include <boost/test/unit_test.hpp>

#include <js_signed_right_shift.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_js_signed_right_shift) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    js_signed_right_shift_t* r = new js_signed_right_shift_t(&ks);

    BOOST_CHECK_EQUAL(r->should_be_40000000(), 0x40000000);
    BOOST_CHECK_EQUAL(r->should_be_a00000(), 0xa00000);

    delete r;
}
