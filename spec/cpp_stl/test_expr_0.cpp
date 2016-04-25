#include <boost/test/unit_test.hpp>

#include <expr_0.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_expr_0) {
    std::ifstream ifs("src/str_encodings.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    expr_0_t* r = new expr_0_t(&ks);

    BOOST_CHECK_EQUAL(r->must_be_f7(), 0xf7);
    BOOST_CHECK_EQUAL(r->must_be_abc123(), "abc123");

    delete r;
}
