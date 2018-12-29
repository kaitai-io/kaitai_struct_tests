#include <boost/test/unit_test.hpp>

#include <imports0.h>
#include <hello_world.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_imports0) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    imports0_t* r = new imports0_t(&ks);

    BOOST_CHECK_EQUAL(r->two(), 0x50);
    BOOST_CHECK_EQUAL(r->hw()->one(), 0x41);
    BOOST_CHECK_EQUAL(r->hw_one(), 0x41);

    delete r;
}
