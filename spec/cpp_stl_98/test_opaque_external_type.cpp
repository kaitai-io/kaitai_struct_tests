#include <boost/test/unit_test.hpp>
#include "opaque_external_type.h"
#include "hello_world.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_opaque_external_type) {
    std::ifstream ifs("src/term_strz.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    opaque_external_type_t* r = new opaque_external_type_t(&ks);

    BOOST_CHECK_EQUAL(r->hw()->one(), 102);

    delete r;
}
