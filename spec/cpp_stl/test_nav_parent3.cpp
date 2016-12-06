#include <boost/test/unit_test.hpp>

#include <nav_parent3.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_nav_parent3) {
    std::ifstream ifs("src/nav_parent2.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    nav_parent3_t* r = new nav_parent3_t(&ks);

    BOOST_CHECK_EQUAL(r->ofs_tags(), 8);
    BOOST_CHECK_EQUAL(r->num_tags(), 2);

    BOOST_CHECK_EQUAL(r->tags()->at(0)->name(), "RAHC");
    BOOST_CHECK_EQUAL(r->tags()->at(0)->ofs(), 0x20);
    BOOST_CHECK_EQUAL(r->tags()->at(0)->num_items(), 3);
    BOOST_CHECK_EQUAL(r->tags()->at(0)->tag_content()->content(), "foo");

    BOOST_CHECK_EQUAL(r->tags()->at(1)->name(), "RAHC");
    BOOST_CHECK_EQUAL(r->tags()->at(1)->ofs(), 0x23);
    BOOST_CHECK_EQUAL(r->tags()->at(1)->num_items(), 6);
    BOOST_CHECK_EQUAL(r->tags()->at(1)->tag_content()->content(), "barbaz");

    delete r;
}
