#include <boost/test/unit_test.hpp>

#include <str_literals.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_str_literals) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    str_literals_t* r = new str_literals_t(&ks);

    BOOST_CHECK_EQUAL(r->complex_str(), std::string("\x00\x01\x02\x07\x08\x0a\x0d\x09\x0b\x0c\x1b\x3d\x07\x0a\x24\u263b", 18));
    BOOST_CHECK_EQUAL(r->double_quotes(), "\x22\x22\x22");
    BOOST_CHECK_EQUAL(r->backslashes(), "\x5c\x5c\x5c");
    BOOST_CHECK_EQUAL(r->octal_eatup(), std::string("\x00\x32\x32", 3));
    BOOST_CHECK_EQUAL(r->octal_eatup2(), std::string("\x02\x32", 2));

    delete r;
}
