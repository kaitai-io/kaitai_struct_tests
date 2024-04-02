// Autogenerated from KST: please remove this line if doing any edits by hand!

#include <boost/test/unit_test.hpp>
#include "enum_import_literals.h"
#include "enum_deep.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_enum_import_literals) {
    std::ifstream ifs("src/enum_0.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    enum_import_literals_t* r = new enum_import_literals_t(&ks);

    BOOST_CHECK_EQUAL(r->pet_1_to_i(), 7);
    BOOST_CHECK_EQUAL(r->pet_1_eq(), true);
    BOOST_CHECK_EQUAL(r->pet_2(), enum_deep_t::container1_t::container2_t::ANIMAL_HARE);

    delete r;
}