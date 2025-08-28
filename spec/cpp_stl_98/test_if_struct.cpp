#include <boost/test/unit_test.hpp>
#include "if_struct.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_if_struct) {
    std::ifstream ifs("src/if_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    if_struct_t* r = new if_struct_t(&ks);
    BOOST_CHECK_EQUAL(r->op1()->opcode(), 83);
    BOOST_CHECK(r->op1()->_is_null_arg_tuple());
    BOOST_CHECK(!r->op1()->arg_tuple()); // added manually
    BOOST_CHECK_EQUAL(r->op1()->arg_str()->str(), std::string("foo"));
    BOOST_CHECK_EQUAL(r->op2()->opcode(), 84);
    BOOST_CHECK_EQUAL(r->op2()->arg_tuple()->num1(), 66);
    BOOST_CHECK_EQUAL(r->op2()->arg_tuple()->num2(), 67);
    BOOST_CHECK(r->op2()->_is_null_arg_str());
    BOOST_CHECK(!r->op2()->arg_str()); // added manually
    BOOST_CHECK_EQUAL(r->op3()->opcode(), 83);
    BOOST_CHECK(r->op3()->_is_null_arg_tuple());
    BOOST_CHECK(!r->op3()->arg_tuple()); // added manually
    BOOST_CHECK_EQUAL(r->op3()->arg_str()->str(), std::string("bar"));

    delete r;
}
