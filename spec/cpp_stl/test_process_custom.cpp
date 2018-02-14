#include <boost/test/unit_test.hpp>

#include <process_custom.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_process_custom) {
    std::ifstream ifs("src/process_rotate.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    process_custom_t* r = new process_custom_t(&ks);

    BOOST_CHECK_EQUAL(r->buf1(), std::string("\x10\xb3\x94\x94\xf4", 5));
    BOOST_CHECK_EQUAL(r->buf2(), std::string("\x5f\xba\x7b\x93\x63\x23\x5f", 7));
    BOOST_CHECK_EQUAL(r->buf3(), std::string("\x29\x33\xb1\x38\xb1", 5));

    delete r;
}
