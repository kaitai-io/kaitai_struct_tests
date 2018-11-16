#include <boost/test/unit_test.hpp>

#include <expr_io_eof.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_expr_io_eof) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    expr_io_eof_t* r = new expr_io_eof_t(&ks);

    BOOST_CHECK_EQUAL(r->substream1()->one(), 1262698832);
    // TODO: there should be a check that r->substream1()->two() is
    // undefined, but we still have no public interface in C++ to do
    // that

    BOOST_CHECK_EQUAL(r->substream2()->one(), 4294914349);
    BOOST_CHECK_EQUAL(r->substream2()->two(), 1262698832);

    delete r;
}
