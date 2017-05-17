#include <boost/test/unit_test.hpp>

#include <default_endian_expr_exception.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_default_endian_expr_exception) {
    std::ifstream ifs("src/endian_expr.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);

    BOOST_CHECK_THROW(
        default_endian_expr_exception_t* r = new default_endian_expr_exception_t(&ks),
        std::runtime_error
    );
}
