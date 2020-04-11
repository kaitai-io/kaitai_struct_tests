#include <boost/test/unit_test.hpp>
#include "opaque_external_type_02_parent.h"
#include "opaque_external_type_02_child.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_opaque_external_type_02_parent) {
    std::ifstream ifs("src/term_strz.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    opaque_external_type_02_parent_t* r = new opaque_external_type_02_parent_t(&ks);

    BOOST_CHECK_EQUAL(r->parent()->child()->s1(), std::string("foo"));
    BOOST_CHECK_EQUAL(r->parent()->child()->s2(), std::string("bar"));
    BOOST_CHECK_EQUAL(r->parent()->child()->s3()->s3(), std::string("|baz@"));

    delete r;
}
