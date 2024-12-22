#include <boost/test/unit_test.hpp>
#include "debug_array_user.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_debug_array_user) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    debug_array_user_t* r = new debug_array_user_t(&ks);

    // --debug implies --no-auto-read
    r->_read();

    BOOST_CHECK_EQUAL(r->one_cat()->meow(), 80);
    BOOST_CHECK_EQUAL(r->array_of_cats()->size(), 3);
    BOOST_CHECK_EQUAL(r->array_of_cats()->at(0)->meow(), 65);
    BOOST_CHECK_EQUAL(r->array_of_cats()->at(1)->meow(), 67);
    BOOST_CHECK_EQUAL(r->array_of_cats()->at(2)->meow(), 75);

    delete r;
}
