#include <boost/test/unit_test.hpp>

#include <instance_user_array.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_instance_user_array) {
    std::ifstream ifs("src/instance_std_array.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    instance_user_array_t* r = new instance_user_array_t(&ks);

    BOOST_CHECK_EQUAL(r->ofs(), 0x10);
    BOOST_CHECK_EQUAL(r->qty_entries(), 3);
    BOOST_CHECK_EQUAL(r->entry_size(), 4);

    BOOST_CHECK_EQUAL(r->user_entries()->at(0)->word1(), 0x1111);
    BOOST_CHECK_EQUAL(r->user_entries()->at(0)->word2(), 0x1111);
    BOOST_CHECK_EQUAL(r->user_entries()->at(1)->word1(), 0x2222);
    BOOST_CHECK_EQUAL(r->user_entries()->at(1)->word2(), 0x2222);
    BOOST_CHECK_EQUAL(r->user_entries()->at(2)->word1(), 0x3333);
    BOOST_CHECK_EQUAL(r->user_entries()->at(2)->word2(), 0x3333);

    delete r;
}
