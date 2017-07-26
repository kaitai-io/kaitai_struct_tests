#include <boost/test/unit_test.hpp>

#include <params_call_short.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_params_call_short) {
    std::ifstream ifs("src/term_strz.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    params_call_short_t* r = new params_call_short_t(&ks);

    BOOST_CHECK_EQUAL(r->buf1()->body(), "foo|b");
    BOOST_CHECK_EQUAL(r->buf2()->body(), "ar|ba");
    BOOST_CHECK_EQUAL(r->buf2()->trailer(), 0x7a);

    delete r;
}
