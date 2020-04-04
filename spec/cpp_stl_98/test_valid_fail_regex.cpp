// Autogenerated from KST: please remove this line if doing any edits by hand!

#include <boost/test/unit_test.hpp>
#include "valid_fail_regex.h"
#include <iostream>
#include <fstream>
#include <vector>
#include "kaitai/exceptions.h"

BOOST_AUTO_TEST_CASE(test_valid_fail_regex) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    valid_fail_regex_t* r = 0;
    BOOST_CHECK_THROW(
        r = new valid_fail_regex_t(&ks),
        kaitai::validation_regex_match_error<std::string>
    );

    delete r;
}
