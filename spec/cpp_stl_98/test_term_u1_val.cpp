// Autogenerated from KST: please remove this line if doing any edits by hand!

#include <boost/test/unit_test.hpp>
#include "term_u1_val.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_term_u1_val) {
    std::ifstream ifs("src/str_encodings.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    term_u1_val_t* r = new term_u1_val_t(&ks);

    BOOST_CHECK_EQUAL(r->foo(), std::string("\x0A\x00\x53\x6F\x6D\x65\x20\x41\x53\x43\x49\x49\x0F\x00", 14));
    BOOST_CHECK_EQUAL(r->bar(), std::string("\u3053\u3093\u306b"));

    delete r;
}
