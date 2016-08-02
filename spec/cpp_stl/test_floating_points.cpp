#include <boost/test/unit_test.hpp>

#include <floating_points.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_floating_points) {
    std::ifstream ifs("src/floating_points.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    floating_points_t* r = new floating_points_t(&ks);

    double delta = 1e-6;

    BOOST_CHECK_EQUAL(r->single_value(), 0.5f);
    BOOST_CHECK_EQUAL(r->single_value_be(), 0.5f);

    BOOST_CHECK_EQUAL(r->double_value(), 0.25d);
    BOOST_CHECK_EQUAL(r->double_value_be(), 0.25d);

    //assertEquals(1.2345d, fp.approximateValue(), delta);

    //assertEquals(1.5f, fp.singleValuePlusInt(), delta);
    //assertEquals(1.0d, fp.singleValuePlusFloat(), delta);
    //assertEquals(0.3d, fp.doubleValuePlusFloat(), delta);

    delete r;
}
