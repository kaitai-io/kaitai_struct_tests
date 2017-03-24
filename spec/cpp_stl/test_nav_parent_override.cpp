#include <boost/test/unit_test.hpp>

#include <nav_parent_override.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_nav_parent_override) {
    std::ifstream ifs("src/nav_parent_codes.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    nav_parent_override_t* r = new nav_parent_override_t(&ks);

    BOOST_CHECK_EQUAL(r->child_size(), 3);
    BOOST_CHECK_EQUAL(r->child_1()->data(), std::string("I12"));
    BOOST_CHECK_EQUAL(r->mediator_2()->child_2()->data(), std::string("3Bb"));

    delete r;
}
