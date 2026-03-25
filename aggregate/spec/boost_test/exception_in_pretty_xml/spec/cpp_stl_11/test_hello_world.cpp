#include <boost/test/unit_test.hpp>
#include "hello_world.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_hello_world) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    hello_world_t r(&ks);

    BOOST_CHECK_EQUAL(r.one(), 80);
    throw std::runtime_error("special sequence ]]> in a CDATA section");
}
