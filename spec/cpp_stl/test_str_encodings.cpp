#include <boost/test/unit_test.hpp>

#include <str_encodings.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_str_encodings) {
    std::ifstream ifs("src/str_encodings.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    str_encodings_t* r = new str_encodings_t(&ks);

    BOOST_CHECK_EQUAL(r->str1(), "Some ASCII");
    BOOST_CHECK_EQUAL(r->str2(), "こんにちは");
    BOOST_CHECK_EQUAL(r->str3(), "こんにちは");
    BOOST_CHECK_EQUAL(r->str4(), "░▒▓");

    delete r;
}
