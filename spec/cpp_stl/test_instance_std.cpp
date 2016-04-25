#include <boost/test/unit_test.hpp>

#include <instance_std.h>

#include <iostream>
#include <fstream>

BOOST_AUTO_TEST_CASE(test_instance_std) {
    std::ifstream ifs("src/str_encodings.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    instance_std_t* r = new instance_std_t(&ks);

    BOOST_CHECK_EQUAL(r->header(), "Some ");

    delete r;
}
