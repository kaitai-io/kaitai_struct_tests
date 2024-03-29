// Autogenerated from KST: please remove this line if doing any edits by hand!

#include <boost/test/unit_test.hpp>
#include "bits_unaligned_b32_be.h"
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_bits_unaligned_b32_be) {
    std::ifstream ifs("src/process_xor_4.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    bits_unaligned_b32_be_t* r = new bits_unaligned_b32_be_t(&ks);

    BOOST_CHECK_EQUAL(r->a(), true);
    BOOST_CHECK_EQUAL(r->b(), 3648472617UL);
    BOOST_CHECK_EQUAL(r->c(), 10);

    delete r;
}
