#include <boost/test/unit_test.hpp>

#include <cast_to_imported.h>
#include <hello_world.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_cast_to_imported) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    cast_to_imported_t* r = new cast_to_imported_t(&ks);

    BOOST_CHECK_EQUAL(r->one()->one(), 0x50);
    BOOST_CHECK_EQUAL(r->one_casted()->one(), 0x50);

    delete r;
}
