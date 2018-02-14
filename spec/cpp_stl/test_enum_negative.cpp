#include <boost/test/unit_test.hpp>

#include <enum_negative.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_enum_negative) {
    std::ifstream ifs("src/enum_negative.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    enum_negative_t* r = new enum_negative_t(&ks);

    BOOST_CHECK_EQUAL(r->f1(), enum_negative_t::CONSTANTS_NEGATIVE_ONE);
    BOOST_CHECK_EQUAL(r->f2(), enum_negative_t::CONSTANTS_POSITIVE_ONE);

    delete r;
}
