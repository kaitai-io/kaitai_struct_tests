#include <boost/test/unit_test.hpp>

#include <expr_2.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_expr_2) {
    std::ifstream ifs("src/str_encodings.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    expr_2_t* r = new expr_2_t(&ks);

    BOOST_CHECK_EQUAL(r->str1()->len_orig(), 10);
    BOOST_CHECK_EQUAL(r->str1()->len_mod(), 7);
    BOOST_CHECK_EQUAL(r->str1()->str(), "Some AS");

    BOOST_CHECK_EQUAL(r->str1_len(), 7);
    BOOST_CHECK_EQUAL(r->str1_len_mod(), 7);
    BOOST_CHECK_EQUAL(r->str1_byte1(), 0x49);
    BOOST_CHECK_EQUAL(r->str1_avg(), 0x49);
    BOOST_CHECK_EQUAL(r->str1_char5(), "e");

    BOOST_CHECK_EQUAL(r->str1_tuple5()->byte0(), 0x65);
    BOOST_CHECK_EQUAL(r->str1_tuple5()->byte1(), 0x20);
    BOOST_CHECK_EQUAL(r->str1_tuple5()->byte2(), 0x41);
    BOOST_CHECK_EQUAL(r->str1_tuple5()->avg(), 0x30);

    BOOST_CHECK_EQUAL(r->str2_tuple5()->byte0(), 0x65);
    BOOST_CHECK_EQUAL(r->str2_tuple5()->byte1(), 0x20);
    BOOST_CHECK_EQUAL(r->str2_tuple5()->byte2(), 0x41);
    BOOST_CHECK_EQUAL(r->str2_tuple5()->avg(), 0x30);

    delete r;
}
