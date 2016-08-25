#include <boost/test/unit_test.hpp>

#include <repeat_eos_struct.h>

#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_repeat_eos_struct) {
    std::ifstream ifs("src/repeat_eos_struct.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    repeat_eos_struct_t* r = new repeat_eos_struct_t(&ks);

    std::vector<repeat_eos_struct_t::chunk_t*> chunks = *(r->chunks());

    BOOST_CHECK_EQUAL(chunks.size(), 2);
    BOOST_CHECK_EQUAL(chunks[0]->offset(), 0);
    BOOST_CHECK_EQUAL(chunks[0]->len(), 0x42);
    BOOST_CHECK_EQUAL(chunks[1]->offset(), 0x42);
    BOOST_CHECK_EQUAL(chunks[1]->len(), 0x815);

    delete r;
}
