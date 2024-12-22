#include <boost/test/unit_test.hpp>
#include "debug_switch_user.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_debug_switch_user) {
    std::ifstream ifs("src/nav_parent_switch.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    debug_switch_user_t* r = new debug_switch_user_t(&ks);

    // --debug implies --no-auto-read
    r->_read();

    BOOST_CHECK_EQUAL(r->code(), 1);
    BOOST_CHECK_EQUAL(static_cast<debug_switch_user_t::one_t*>(r->data())->val(), -190);

    delete r;
}
