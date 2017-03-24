#include <boost/test/unit_test.hpp>

#include <bits_byte_aligned.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_bits_byte_aligned) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    bits_byte_aligned_t* r = new bits_byte_aligned_t(&ks);

    // 50 (6 + 2) = 010100|00
    BOOST_CHECK_EQUAL(r->one(), 0b010100);
    // 41
    BOOST_CHECK_EQUAL(r->byte_1(), 0x41);
    // 43 (3 + 1 + 4) = 010|0|0011
    BOOST_CHECK_EQUAL(r->two(), 0b010);
    BOOST_CHECK_EQUAL(r->three(), false);
    // 4B
    BOOST_CHECK_EQUAL(r->byte_2(), 0x4b);
    // 2D 31 (14 + 2) = 00101101 001100|01
    BOOST_CHECK_EQUAL(r->four(), 0b00101101001100);
    // FF
    BOOST_CHECK_EQUAL(r->byte_3(), "\xff");
    // FF
    BOOST_CHECK_EQUAL(r->full_byte(), 0xff);
    // 50
    BOOST_CHECK_EQUAL(r->byte_4(), 0x50);

    delete r;
}
