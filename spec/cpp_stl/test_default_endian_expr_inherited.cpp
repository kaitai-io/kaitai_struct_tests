#include <boost/test/unit_test.hpp>

#include <default_endian_expr_inherited.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_default_endian_expr_inherited) {
    std::ifstream ifs("src/endian_expr.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    default_endian_expr_inherited_t* r = new default_endian_expr_inherited_t(&ks);

    BOOST_CHECK_EQUAL(r->docs()->at(0)->indicator(), "II");
    BOOST_CHECK_EQUAL(r->docs()->at(0)->main()->insides()->some_int(), 0x42);
    BOOST_CHECK_EQUAL(r->docs()->at(0)->main()->insides()->more()->some_int1(), 0x4200);
    BOOST_CHECK_EQUAL(r->docs()->at(0)->main()->insides()->more()->some_int2(), 0x42);
    BOOST_CHECK_EQUAL(r->docs()->at(0)->main()->insides()->more()->some_inst(), 0x42);

    BOOST_CHECK_EQUAL(r->docs()->at(1)->indicator(), "MM");
    BOOST_CHECK_EQUAL(r->docs()->at(1)->main()->insides()->some_int(), 0x42);
    BOOST_CHECK_EQUAL(r->docs()->at(1)->main()->insides()->more()->some_int1(), 0x42);
    BOOST_CHECK_EQUAL(r->docs()->at(1)->main()->insides()->more()->some_int2(), 0x4200);
    BOOST_CHECK_EQUAL(r->docs()->at(1)->main()->insides()->more()->some_inst(), 0x42000000);

    BOOST_CHECK_EQUAL(r->docs()->at(2)->indicator(), "XX");
    BOOST_CHECK_EQUAL(r->docs()->at(2)->main()->insides()->some_int(), 0x42);
    BOOST_CHECK_EQUAL(r->docs()->at(2)->main()->insides()->more()->some_int1(), 0x42);
    BOOST_CHECK_EQUAL(r->docs()->at(2)->main()->insides()->more()->some_int2(), 0x4200);
    BOOST_CHECK_EQUAL(r->docs()->at(2)->main()->insides()->more()->some_inst(), 0x42000000);

    delete r;
}
