#include <boost/test/unit_test.hpp>

#include <position_in_seq.h>

#include <iostream>
#include <fstream>

BOOST_AUTO_TEST_CASE(test_position_in_seq) {
    std::ifstream ifs("src/position_in_seq.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    position_in_seq_t* r = new position_in_seq_t(&ks);

    uint32_t exp_array[] = {1, 2, 3};
    std::vector<uint32_t> exp_vector(exp_array, exp_array + sizeof(exp_array) / sizeof(uint32_t));

    BOOST_CHECK_EQUAL(r->header()->qty_numbers(), 3);
    BOOST_CHECK_EQUAL_COLLECTIONS(
        r->numbers()->begin(), r->numbers()->end(),
        exp_vector.begin(), exp_vector.end()
    );

    delete r;
}
