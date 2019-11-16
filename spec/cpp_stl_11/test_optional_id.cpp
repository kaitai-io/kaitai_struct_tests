#include <boost/test/unit_test.hpp>

#include <optional_id.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_optional_id) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    optional_id_t* r = new optional_id_t(&ks);

    BOOST_CHECK_EQUAL(r->_unnamed0(), 0x50);
    BOOST_CHECK_EQUAL(r->_unnamed1(), 0x41);
    BOOST_CHECK_EQUAL(r->_unnamed2(), "\x43\x4b\x2d\x31\xff");

    delete r;
}
