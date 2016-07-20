#include <boost/test/unit_test.hpp>

#include <process_xor_value.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_process_xor_value) {
    std::ifstream ifs("src/process_xor_1.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    process_xor_value_t* r = new process_xor_value_t(&ks);

    BOOST_CHECK_EQUAL(r->key(), 0xff);
    BOOST_CHECK_EQUAL(r->buf(), "foo bar");

    delete r;
}
