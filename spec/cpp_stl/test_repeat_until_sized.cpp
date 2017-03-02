#include <boost/test/unit_test.hpp>

#include <repeat_until_sized.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_repeat_until_sized) {
    std::ifstream ifs("src/repeat_until_process.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    repeat_until_sized_t* r = new repeat_until_sized_t(&ks);

    BOOST_CHECK_EQUAL(r->records()->size(), 3);

    BOOST_CHECK_EQUAL(r->records()->at(0)->marker(), 0xe8);
    BOOST_CHECK_EQUAL(r->records()->at(0)->body(), 0xaaaaaaba);

    BOOST_CHECK_EQUAL(r->records()->at(1)->marker(), 0xfa);
    BOOST_CHECK_EQUAL(r->records()->at(1)->body(), 0xaaaab89e);

    BOOST_CHECK_EQUAL(r->records()->at(2)->marker(), 0xaa);
    BOOST_CHECK_EQUAL(r->records()->at(2)->body(), 0x55555555);

    delete r;
}
