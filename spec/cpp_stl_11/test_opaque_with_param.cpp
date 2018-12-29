#include <boost/test/unit_test.hpp>

#include <opaque_with_param.h>
#include <params_def.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_opaque_with_param) {
    std::ifstream ifs("src/term_strz.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    opaque_with_param_t* r = new opaque_with_param_t(&ks);

    BOOST_CHECK_EQUAL(r->one()->buf(), "foo|b");
    BOOST_CHECK_EQUAL(r->one()->trailer(), 0x61);

    delete r;
}
