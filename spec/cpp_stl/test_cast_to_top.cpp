#include <boost/test/unit_test.hpp>

#include <cast_to_top.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_cast_to_top) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    cast_to_top_t* r = new cast_to_top_t(&ks);

    BOOST_CHECK_EQUAL(r->code(), 0x50);
    BOOST_CHECK_EQUAL(r->header()->code(), 0x41);
    BOOST_CHECK_EQUAL(r->header_casted()->code(), 0x41);

    delete r;
}
