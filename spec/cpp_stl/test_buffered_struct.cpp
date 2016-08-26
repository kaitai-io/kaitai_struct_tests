#include <boost/test/unit_test.hpp>

#include <buffered_struct.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_buffered_struct) {
    std::ifstream ifs("src/buffered_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    buffered_struct_t* r = new buffered_struct_t(&ks);

    BOOST_CHECK_EQUAL(r->len1(), 0x10);
    BOOST_CHECK_EQUAL(r->block1()->number1(), 0x42);
    BOOST_CHECK_EQUAL(r->block1()->number2(), 0x43);
    BOOST_CHECK_EQUAL(r->len2(), 0x8);
    BOOST_CHECK_EQUAL(r->block2()->number1(), 0x44);
    BOOST_CHECK_EQUAL(r->block2()->number2(), 0x45);
    BOOST_CHECK_EQUAL(r->finisher(), 0xee);

    delete r;
}
