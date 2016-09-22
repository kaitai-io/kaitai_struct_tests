#include <boost/test/unit_test.hpp>

#include <nested_same_name.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_nested_same_name) {
    std::ifstream ifs("src/repeat_n_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    nested_same_name_t* r = new nested_same_name_t(&ks);

    BOOST_CHECK_EQUAL(r->main_data()->main_size(), 2);
    BOOST_CHECK_EQUAL(r->main_data()->foo()->data(), std::string("\x10\0\0\0", 4));

    delete r;
}
