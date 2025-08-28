#include <boost/test/unit_test.hpp>
#include "type_ternary_2nd_falsy.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_type_ternary_2nd_falsy) {
    std::ifstream ifs("src/switch_integers.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    type_ternary_2nd_falsy_t* r = new type_ternary_2nd_falsy_t(&ks);

    BOOST_CHECK_EQUAL(r->v_false(), false);
    BOOST_CHECK_EQUAL(r->v_int_zero(), 0);
    BOOST_CHECK_EQUAL(r->v_int_neg_zero(), -0);
    BOOST_CHECK_CLOSE(r->v_float_zero(), 0.0, 1e-4);
    BOOST_CHECK_CLOSE(r->v_float_neg_zero(), -0.0, 1e-4);
    BOOST_CHECK_EQUAL(r->v_str_w_zero(), std::string("0"));
    BOOST_CHECK_EQUAL(r->v_str_w_zero().length(), 1);
    BOOST_CHECK_EQUAL(r->ut()->m(), 7);
    // BOOST_CHECK(r->_is_null_v_null_ut());
    BOOST_CHECK_EQUAL(r->v_str_empty(), std::string(""));
    BOOST_CHECK_EQUAL(r->v_str_empty().length(), 0);
    BOOST_CHECK_EQUAL(r->int_array()->size(), 2);
    BOOST_CHECK_EQUAL(r->v_int_array_empty()->size(), 0);

    delete r;
}
