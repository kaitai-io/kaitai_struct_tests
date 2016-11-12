#include <boost/test/unit_test.hpp>

#include <nav_parent_switch.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_nav_parent_switch) {
    std::ifstream ifs("src/nav_parent_switch.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    nav_parent_switch_t* r = new nav_parent_switch_t(&ks);

    BOOST_CHECK_EQUAL(r->category(), 1);
    nav_parent_switch_t::element_1_t* content = static_cast<nav_parent_switch_t::element_1_t*> (r->content());
    BOOST_CHECK_EQUAL(content->foo(), 0x42);
    BOOST_CHECK_EQUAL(content->subelement()->bar(), 0xff);

    delete r;
}
