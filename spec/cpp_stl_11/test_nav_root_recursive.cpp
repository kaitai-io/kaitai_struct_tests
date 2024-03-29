// Autogenerated from KST: please remove this line if doing any edits by hand!

#include <boost/test/unit_test.hpp>
#include "nav_root_recursive.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_nav_root_recursive) {
    std::ifstream ifs("src/enum_negative.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    nav_root_recursive_t* r = new nav_root_recursive_t(&ks);

    BOOST_CHECK_EQUAL(r->value(), 255);
    BOOST_CHECK_EQUAL(r->next()->value(), 1);
    BOOST_CHECK_EQUAL(r->next()->root_value(), 255);
    BOOST_CHECK(r->next()->_is_null_next());

    delete r;
}
