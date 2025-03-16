// Autogenerated from KST: please remove this line if doing any edits by hand!

#include <boost/test/unit_test.hpp>
#include "eos_exception_unsigned_loop.h"
#include <iostream>
#include <fstream>
#include <vector>
#include "kaitai/exceptions.h"

BOOST_AUTO_TEST_CASE(test_eos_exception_unsigned_loop) {
    std::ifstream ifs("src/high_bit_u4_loop_count.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    eos_exception_unsigned_loop_t* r = nullptr;
    BOOST_CHECK_THROW(
        r = new eos_exception_unsigned_loop_t(&ks),
        std::ifstream::failure
    );

    delete r;
}
