#include <boost/test/unit_test.hpp>
#include "debug_array_user_current_excluded.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_debug_array_user_current_excluded) {
    std::ifstream ifs("src/term_strz.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    debug_array_user_current_excluded_t* r = new debug_array_user_current_excluded_t(&ks);

    // --debug implies --no-auto-read
    r->_read();

    BOOST_CHECK_EQUAL(r->array_of_cats()->at(0)->meow(), std::string("\x66\x6F\x6F", 3));
    BOOST_CHECK_EQUAL(r->array_of_cats()->at(1)->meow(), std::string("\x7C\x62", 2));
    BOOST_CHECK_EQUAL(r->array_of_cats()->at(2)->meow(), std::string("\x61", 1));

    delete r;
}
