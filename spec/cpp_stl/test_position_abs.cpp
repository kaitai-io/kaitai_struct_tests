#include <boost/test/unit_test.hpp>

#include <position_abs.h>

#include <iostream>
#include <fstream>

BOOST_AUTO_TEST_CASE(test_position_abs) {
    std::ifstream ifs("src/position_abs.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    position_abs_t* r = new position_abs_t(&ks);

    BOOST_CHECK_EQUAL(r->index_offset(), 0x20);
    BOOST_CHECK_EQUAL(r->index()->entry(), "foo");

    delete r;
}
