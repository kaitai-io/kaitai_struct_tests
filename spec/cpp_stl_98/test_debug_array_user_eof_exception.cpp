#include <boost/test/unit_test.hpp>
#include "debug_array_user_eof_exception.h"
#include <iostream>
#include <fstream>
#include <vector>
#include "kaitai/exceptions.h"

BOOST_AUTO_TEST_CASE(test_debug_array_user_eof_exception) {
    std::ifstream ifs("src/nav_parent_codes.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    debug_array_user_eof_exception_t* r = new debug_array_user_eof_exception_t(&ks);

    // --debug implies --no-auto-read
    BOOST_CHECK_THROW(
        r->_read(),
        std::ifstream::failure
    );

    BOOST_CHECK_EQUAL(r->one_cat()->meow(), 3);
    BOOST_CHECK_EQUAL(r->one_cat()->chirp(), 73);
    BOOST_CHECK_EQUAL(r->array_of_cats()->size(), 3);
    BOOST_CHECK_EQUAL(r->array_of_cats()->at(0)->meow(), 49);
    BOOST_CHECK_EQUAL(r->array_of_cats()->at(0)->chirp(), 50);
    BOOST_CHECK_EQUAL(r->array_of_cats()->at(1)->meow(), 51);
    BOOST_CHECK_EQUAL(r->array_of_cats()->at(1)->chirp(), 66);
    BOOST_CHECK_EQUAL(r->array_of_cats()->at(2)->meow(), 98);

    delete r;
}
