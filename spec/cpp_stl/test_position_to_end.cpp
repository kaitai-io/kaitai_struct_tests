#include <boost/test/unit_test.hpp>

#include <position_to_end.h>

#include <iostream>
#include <fstream>

BOOST_AUTO_TEST_CASE(test_position_to_end) {
    std::ifstream ifs("src/position_to_end.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    position_to_end_t* r = new position_to_end_t(&ks);

    BOOST_CHECK_EQUAL(r->index()->foo(), 0x42);
    BOOST_CHECK_EQUAL(r->index()->bar(), 0x1234);

    delete r;
}
