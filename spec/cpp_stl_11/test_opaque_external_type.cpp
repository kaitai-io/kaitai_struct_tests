#include <boost/test/unit_test.hpp>

#include <opaque_external_type.h>
#include <term_strz.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_opaque_external_type) {
    std::ifstream ifs("src/term_strz.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    opaque_external_type_t* r = new opaque_external_type_t(&ks);

    BOOST_CHECK_EQUAL(r->one()->s1(), "foo");
    BOOST_CHECK_EQUAL(r->one()->s2(), "bar");
    BOOST_CHECK_EQUAL(r->one()->s3(), "|baz@");

    delete r;
}
