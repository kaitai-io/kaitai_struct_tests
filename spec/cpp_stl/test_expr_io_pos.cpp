#include <boost/test/unit_test.hpp>

#include <expr_io_pos.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_expr_io_pos) {
    std::ifstream ifs("src/expr_io_pos.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    expr_io_pos_t* r = new expr_io_pos_t(&ks);

    BOOST_CHECK_EQUAL(r->substream1()->my_str(), "CURIOSITY");
    BOOST_CHECK_EQUAL(r->substream1()->body(), std::string("\x11\x22\x33\x44"));
    BOOST_CHECK_EQUAL(r->substream1()->number(), 0x42);

    BOOST_CHECK_EQUAL(r->substream2()->my_str(), "KILLED");
    BOOST_CHECK_EQUAL(r->substream2()->body(), std::string("a cat"));
    BOOST_CHECK_EQUAL(r->substream2()->number(), 0x67);

    delete r;
}
