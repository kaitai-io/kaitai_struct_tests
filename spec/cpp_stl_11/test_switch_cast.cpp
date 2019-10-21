#include <boost/test/unit_test.hpp>

#include <switch_cast.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_switch_cast) {
    std::ifstream ifs("src/switch_opcodes.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    switch_cast_t* r = new switch_cast_t(&ks);

    BOOST_CHECK_EQUAL(r->first_obj()->value(), "foobar");
    BOOST_CHECK_EQUAL(r->second_val(), 0x42);

    // Unable to test "err_cast": C++ static casts with pointer types
    // don't do any type checking and do not throw relevant
    // std::bad_cast.

    delete r;
}
