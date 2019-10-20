#include <boost/test/unit_test.hpp>

#include <eos_exception_u4.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_eos_exception_u4) {
    std::ifstream ifs("src/term_strz.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);

    BOOST_CHECK_THROW(
        eos_exception_u4_t* r = new eos_exception_u4_t(&ks),
        std::ifstream::failure
    );
}
