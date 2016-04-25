#include <boost/test/unit_test.hpp>

#include <repeat_n_strz.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_repeat_n_strz) {
    std::ifstream ifs("src/repeat_n_strz.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    repeat_n_strz_t* r = new repeat_n_strz_t(&ks);

    BOOST_CHECK_EQUAL(r->qty(), 2);

    std::vector<std::string> exp_vector;
    exp_vector.push_back(std::string("foo"));
    exp_vector.push_back(std::string("bar"));
    
    BOOST_CHECK_EQUAL_COLLECTIONS(
        r->lines()->begin(), r->lines()->end(),
        exp_vector.begin(), exp_vector.end()
    );
}
