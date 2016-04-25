#include <boost/test/unit_test.hpp>

#include <hello_world.h>

#include <iostream>
#include <fstream>

BOOST_AUTO_TEST_CASE(test_hello_world) {
//    hello_world* r = hello_world::from_file("src/fixed_struct.bin");

    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    hello_world_t* r = new hello_world_t(&ks);

    BOOST_CHECK_EQUAL(r->one(), 0x50);

    delete r;
}
