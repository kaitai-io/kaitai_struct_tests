#include <boost/test/unit_test.hpp>
#include "debug_0.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_debug_0) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    debug_0_t* r = new debug_0_t(&ks);

    // --debug implies --no-auto-read
    r->_read();

    BOOST_CHECK_EQUAL(r->one(), 80);
    BOOST_CHECK_EQUAL(r->array_of_ints()->size(), 3);
    BOOST_CHECK_EQUAL(r->array_of_ints()->at(0), 65);
    BOOST_CHECK_EQUAL(r->array_of_ints()->at(1), 67);
    BOOST_CHECK_EQUAL(r->array_of_ints()->at(2), 75);
    BOOST_CHECK_EQUAL(r->_unnamed2(), 45);

    // FIXME: also test --read-pos once it is implemented

    delete r;
}
