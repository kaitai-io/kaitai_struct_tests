#include <boost/test/unit_test.hpp>

#include <repeat_until_s4.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_repeat_until_s4) {
    std::ifstream ifs("src/repeat_until_s4.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    repeat_until_s4_t* r = new repeat_until_s4_t(&ks);

    std::vector<int32_t> exp_vector;
    exp_vector.push_back(0x42);
    exp_vector.push_back(0x1337);
    exp_vector.push_back(-251658241);
    exp_vector.push_back(-1);

    BOOST_CHECK_EQUAL_COLLECTIONS(
        r->entries()->begin(), r->entries()->end(),
        exp_vector.begin(), exp_vector.end()
    );
    BOOST_CHECK_EQUAL(r->afterall(), std::string("foobar"));

    delete r;
}
