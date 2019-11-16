#include <boost/test/unit_test.hpp>

#include <params_def.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_params_def) {
    std::ifstream ifs("src/term_strz.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    params_def_t* r = new params_def_t(5, true, &ks);

    BOOST_CHECK_EQUAL(r->buf(), "foo|b");
    BOOST_CHECK_EQUAL(r->trailer(), 0x61);

    delete r;
}
