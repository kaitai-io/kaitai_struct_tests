#include <boost/test/unit_test.hpp>

#include <imports_circular_a.h>
#include <imports_circular_b.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_imports_circular_a) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    imports_circular_a_t* r = new imports_circular_a_t(&ks);

    BOOST_CHECK_EQUAL(r->code(), 0x50);
    BOOST_CHECK_EQUAL(r->two()->initial(), 0x41);
    BOOST_CHECK_EQUAL(r->two()->back_ref()->code(), 0x43);
    BOOST_CHECK_EQUAL(r->two()->back_ref()->two()->initial(), 0x4b);
    BOOST_CHECK_EQUAL(r->two()->back_ref()->two()->_is_null_back_ref(), true);

    delete r;
}
