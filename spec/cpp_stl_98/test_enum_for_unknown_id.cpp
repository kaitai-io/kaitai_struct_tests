#include <boost/test/unit_test.hpp>

#include <enum_for_unknown_id.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_enum_for_unknown_id) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    enum_for_unknown_id_t* r = new enum_for_unknown_id_t(&ks);

    BOOST_CHECK_EQUAL(r->one(), 80);

    delete r;
}
