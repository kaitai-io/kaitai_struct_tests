#include <boost/test/unit_test.hpp>

#include <process_rotate.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_process_rotate) {
    std::ifstream ifs("src/process_rotate.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    process_rotate_t* r = new process_rotate_t(&ks);

    BOOST_CHECK_EQUAL(r->buf1(), "Hello");
    BOOST_CHECK_EQUAL(r->buf2(), "World");
    BOOST_CHECK_EQUAL(r->buf3(), "There");

    delete r;
}
