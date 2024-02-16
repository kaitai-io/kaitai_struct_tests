#include <boost/test/unit_test.hpp>
#include "to_string_custom.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_to_string_custom) {
    std::ifstream ifs("src/term_strz.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    to_string_custom_t* r = new to_string_custom_t(&ks);

    std::stringstream ss;
    ss << *r;

    BOOST_CHECK_EQUAL(ss.str(), "s1 = foo, s2 = bar");

    delete r;
}
