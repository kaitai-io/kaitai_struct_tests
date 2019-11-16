#include <boost/test/unit_test.hpp>

#include <imports_abs_abs.h>
#include <imported_and_abs.h>
#include <imported_root.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_imports_abs_abs) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    imports_abs_abs_t* r = new imports_abs_abs_t(&ks);

    BOOST_CHECK_EQUAL(r->one(), 0x50);
    BOOST_CHECK_EQUAL(r->two()->one(), 0x41);
    BOOST_CHECK_EQUAL(r->two()->two()->one(), 0x43);

    delete r;
}
