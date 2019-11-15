#include <boost/test/unit_test.hpp>

#include <type_ternary_opaque.h>
#include <term_strz.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_type_ternary_opaque) {
    std::ifstream ifs("src/term_strz.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    type_ternary_opaque_t* r = new type_ternary_opaque_t(&ks);

    BOOST_CHECK_EQUAL(r->dif()->s1(), "foo");
    BOOST_CHECK_EQUAL(r->dif()->s2(), "bar");
    BOOST_CHECK_EQUAL(r->dif()->s3(), "|baz@");

    delete r;
}
