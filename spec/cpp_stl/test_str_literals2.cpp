#include <boost/test/unit_test.hpp>

#include <str_literals2.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_str_literals2) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    str_literals2_t* r = new str_literals2_t(&ks);

    BOOST_CHECK_EQUAL(r->dollar1(), "$foo");
    BOOST_CHECK_EQUAL(r->dollar2(), "${foo}");
    BOOST_CHECK_EQUAL(r->hash(), "#{foo}");
    BOOST_CHECK_EQUAL(r->at_sign(), "@foo");

    delete r;
}
