#include <boost/test/unit_test.hpp>
#include "nested_types_import.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_nested_types_import) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    nested_types_import_t* r = new nested_types_import_t(&ks);

    BOOST_CHECK_EQUAL(r->a_cc()->value_cc(), 80);
    BOOST_CHECK_EQUAL(r->a_c_d()->value_d(), 65);
    BOOST_CHECK_EQUAL(r->b()->value_b(), 67);
    BOOST_CHECK_EQUAL(r->b()->a_cc()->value_cc(), 75);
    BOOST_CHECK_EQUAL(r->b()->a_c_d()->value_d(), 45);
    BOOST_CHECK_EQUAL(r->a_cc()->_parent(), nullptr);
    BOOST_CHECK_EQUAL(r->a_cc()->_root(), nullptr);
    BOOST_CHECK_EQUAL(r->a_c_d()->_parent(), nullptr);
    BOOST_CHECK_EQUAL(r->a_c_d()->_root(), nullptr);
    BOOST_CHECK_EQUAL(r->b()->_parent(), nullptr);
    BOOST_CHECK_EQUAL(r->b()->_root(), nullptr);

    delete r;
}
