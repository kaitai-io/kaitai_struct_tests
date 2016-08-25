#include <boost/test/unit_test.hpp>

#include <repeat_n_struct.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_repeat_n_struct) {
    std::ifstream ifs("src/repeat_n_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    repeat_n_struct_t* r = new repeat_n_struct_t(&ks);

    BOOST_CHECK_EQUAL(r->qty(), 2);

    BOOST_CHECK_EQUAL(r->chunks()->at(0)->offset(), 0x10);
    BOOST_CHECK_EQUAL(r->chunks()->at(0)->len(), 0x2078);
    BOOST_CHECK_EQUAL(r->chunks()->at(1)->offset(), 0x2088);
    BOOST_CHECK_EQUAL(r->chunks()->at(1)->len(), 0xf);
}
