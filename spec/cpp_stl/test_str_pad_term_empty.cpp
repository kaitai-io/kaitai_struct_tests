#include <boost/test/unit_test.hpp>

#include <str_pad_term_empty.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_str_pad_term_empty) {
    std::ifstream ifs("src/str_pad_term_empty.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    str_pad_term_empty_t* r = new str_pad_term_empty_t(&ks);

    BOOST_CHECK_EQUAL(r->str_pad(), "");
    BOOST_CHECK_EQUAL(r->str_term(), "");
    BOOST_CHECK_EQUAL(r->str_term_and_pad(), "");
    BOOST_CHECK_EQUAL(r->str_term_include(), "@");

    delete r;
}
