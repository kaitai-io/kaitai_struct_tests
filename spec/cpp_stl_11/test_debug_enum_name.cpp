#include <boost/test/unit_test.hpp>
#include "debug_enum_name.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_debug_enum_name) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    debug_enum_name_t* r = new debug_enum_name_t(&ks);

    // --debug implies --no-auto-read
    r->_read();

    BOOST_CHECK_EQUAL(r->one(), debug_enum_name_t::TEST_ENUM1_ENUM_VALUE_80);
    BOOST_CHECK_EQUAL(r->array_of_ints()->at(0), debug_enum_name_t::TEST_ENUM2_ENUM_VALUE_65);
    BOOST_CHECK_EQUAL(r->test_type()->field1(), debug_enum_name_t::test_subtype_t::INNER_ENUM1_ENUM_VALUE_67);
    BOOST_CHECK_EQUAL(r->test_type()->instance_field(), debug_enum_name_t::test_subtype_t::INNER_ENUM2_ENUM_VALUE_11);

    delete r;
}
