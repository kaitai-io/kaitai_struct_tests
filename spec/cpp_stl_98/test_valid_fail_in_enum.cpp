#include <boost/test/unit_test.hpp>
#include "valid_fail_in_enum.h"
#include <iostream>
#include <fstream>
#include <vector>
#include "kaitai/exceptions.h"

BOOST_AUTO_TEST_CASE(test_valid_fail_in_enum) {
    std::ifstream ifs("src/enum_0.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    valid_fail_in_enum_t* r = 0;
    BOOST_CHECK_THROW(
        r = new valid_fail_in_enum_t(&ks),
        kaitai::validation_not_in_enum_error<valid_fail_in_enum_t::animal_t>
    );

    delete r;
}
