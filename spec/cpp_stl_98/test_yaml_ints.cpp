#include <boost/test/unit_test.hpp>

#include <yaml_ints.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_yaml_ints) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    yaml_ints_t* r = new yaml_ints_t(&ks);

    BOOST_CHECK_EQUAL(r->test_u4_dec(), 0xffffffff);
    BOOST_CHECK_EQUAL(r->test_u4_hex(), 0xffffffff);
    BOOST_CHECK_EQUAL(r->test_u8_dec(), 0xffffffffffffffffUL);
    BOOST_CHECK_EQUAL(r->test_u8_hex(), 0xffffffffffffffffUL);

    delete r;
}
