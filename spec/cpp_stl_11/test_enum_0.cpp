#include <boost/test/unit_test.hpp>

#include <enum_0.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_enum_0) {
    std::ifstream ifs("src/enum_0.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    enum_0_t* r = new enum_0_t(&ks);

    BOOST_CHECK_EQUAL(r->pet_1(), enum_0_t::ANIMAL_CAT);
    BOOST_CHECK_EQUAL(r->pet_2(), enum_0_t::ANIMAL_CHICKEN);

    delete r;
}
