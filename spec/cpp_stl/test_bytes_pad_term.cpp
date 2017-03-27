#include <boost/test/unit_test.hpp>

#include <bytes_pad_term.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_bytes_pad_term) {
    std::ifstream ifs("src/str_pad_term.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    bytes_pad_term_t* r = new bytes_pad_term_t(&ks);

    BOOST_CHECK_EQUAL(r->str_pad(), "str1");
    BOOST_CHECK_EQUAL(r->str_term(), "str2foo");
    BOOST_CHECK_EQUAL(r->str_term_and_pad(), "str+++3bar+++");
    BOOST_CHECK_EQUAL(r->str_term_include(), "str4baz@");

    delete r;
}
