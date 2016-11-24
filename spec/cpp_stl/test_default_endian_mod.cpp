#include <boost/test/unit_test.hpp>

#include <default_endian_mod.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_default_endian_mod) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    default_endian_mod_t* r = new default_endian_mod_t(&ks);

    BOOST_CHECK_EQUAL(r->main()->one(), 0x4b434150);
    BOOST_CHECK_EQUAL(r->main()->nest()->two(), -52947);
    BOOST_CHECK_EQUAL(r->main()->nest_be()->two(), 0x5041434b);

    delete r;
}
