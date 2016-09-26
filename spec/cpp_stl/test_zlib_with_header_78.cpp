#include <boost/test/unit_test.hpp>

#include <zlib_with_header_78.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_zlib_with_header_78) {
    std::ifstream ifs("src/zlib_with_header_78.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    zlib_with_header_78_t* r = new zlib_with_header_78_t(&ks);

    BOOST_CHECK_EQUAL(r->data(), "a quick brown fox jumps over");

    delete r;
}
