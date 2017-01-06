#include <boost/test/unit_test.hpp>

#include <bits_simple.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_bits_simple) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    bits_simple_t* r = new bits_simple_t(&ks);

    // 50 41
    BOOST_CHECK_EQUAL(r->byte_1(), 0x50);
    BOOST_CHECK_EQUAL(r->byte_2(), 0x41);

    // 43 (1 + 3 + 4) = 0|100|0011
    BOOST_CHECK_EQUAL(r->bits_a(), 0);
    BOOST_CHECK_EQUAL(r->bits_b(), 0b100);
    BOOST_CHECK_EQUAL(r->bits_c(), 0b0011);

    // 4B 2D 31 (10 + 3 + 11) = 01001011 00|101|101 00110001
    BOOST_CHECK_EQUAL(r->large_bits_1(), 0b0100101100);
    BOOST_CHECK_EQUAL(r->spacer(), 0b101);
    BOOST_CHECK_EQUAL(r->large_bits_2(), 0b10100110001);

    // FF FF
    BOOST_CHECK_EQUAL(r->normal_s2(), -1);

    // 50 41 43
    BOOST_CHECK_EQUAL(r->byte_8_9_10(), 0x504143);

    delete r;
}
