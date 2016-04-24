#include <boost/test/unit_test.hpp>

#include <repeat_eos_u4.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_repeat_eos_u4) {
    std::ifstream ifs("src/repeat_eos_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    repeat_eos_u4* r = new repeat_eos_u4(&ks);

    uint32_t exp_array[] = {0, 0x42, 0x42, 0x815};
    std::vector<uint32_t> exp_vector(exp_array, exp_array + sizeof(exp_array) / sizeof(uint32_t));

    BOOST_CHECK_EQUAL_COLLECTIONS(
        r->numbers()->begin(), r->numbers()->end(),
        exp_vector.begin(), exp_vector.end()
    );

    delete r;
}
