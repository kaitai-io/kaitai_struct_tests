#include <boost/test/unit_test.hpp>

#include <recursive_one.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_recursive_one) {
    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    recursive_one_t* r = new recursive_one_t(&ks);

    BOOST_CHECK_EQUAL(r->one(), 0x50);
    recursive_one_t* rec1 = static_cast<recursive_one_t*>(r->next());
    BOOST_CHECK_EQUAL(rec1->one(), 0x41);
    recursive_one_t* rec2 = static_cast<recursive_one_t*>(rec1->next());
    BOOST_CHECK_EQUAL(rec2->one(), 0x43);
    recursive_one_t::fini_t* rec3 = static_cast<recursive_one_t::fini_t*>(rec2->next());
    BOOST_CHECK_EQUAL(rec3->finisher(), 0x2d4b);

    delete r;
}
