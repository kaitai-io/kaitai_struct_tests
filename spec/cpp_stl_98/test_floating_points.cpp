#include <boost/test/unit_test.hpp>

#include <floating_points.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_floating_points) {
    std::ifstream ifs("src/floating_points.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    floating_points_t* r = new floating_points_t(&ks);

    double delta = 1e-5;

    BOOST_CHECK_EQUAL(r->single_value(), 0.5f);
    BOOST_CHECK_EQUAL(r->single_value_be(), 0.5f);

    BOOST_CHECK_EQUAL(r->double_value(), 0.25);
    BOOST_CHECK_EQUAL(r->double_value_be(), 0.25);

    BOOST_CHECK_CLOSE(r->approximate_value(), 1.2345, delta);

    BOOST_CHECK_CLOSE(r->single_value_plus_int(), 1.5f, delta);
    BOOST_CHECK_CLOSE(r->single_value_plus_float(), 1.0, delta);
    BOOST_CHECK_CLOSE(r->double_value_plus_float(), 0.3, delta);

    delete r;
}
