#include <boost/test/unit_test.hpp>

#include <switch_manual_int_size.h>

#include <iostream>
#include <fstream>
#include <vector>

#include "helpers.h"

BOOST_AUTO_TEST_CASE(test_switch_manual_int_size) {
    std::ifstream ifs("src/switch_tlv.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    switch_manual_int_size_t* r = new switch_manual_int_size_t(&ks);

    BOOST_CHECK_EQUAL(r->chunks()->size(), 4);

    BOOST_CHECK_EQUAL(r->chunks()->at(0)->code(), 0x11);
    switch_manual_int_size_t::chunk_t::chunk_meta_t* meta =
        static_cast<switch_manual_int_size_t::chunk_t::chunk_meta_t*>(r->chunks()->at(0)->body());
    BOOST_CHECK_EQUAL(meta->title(), "Stuff");
    BOOST_CHECK_EQUAL(meta->author(), "Me");

    BOOST_CHECK_EQUAL(r->chunks()->at(1)->code(), 0x22);
    switch_manual_int_size_t::chunk_t::chunk_dir_t* dir =
        static_cast<switch_manual_int_size_t::chunk_t::chunk_dir_t*>(r->chunks()->at(1)->body());
    COMPARE_ARRAY(std::string, dir->entries(), "AAAA", "BBBB", "CCCC");

    BOOST_CHECK_EQUAL(r->chunks()->at(2)->code(), 0x33);
    std::string str1 = r->chunks()->at(2)->_raw_body();
    BOOST_CHECK_EQUAL(str1, std::string("\x10\x20\x30\x40\x50\x60\x70\x80"));

    BOOST_CHECK_EQUAL(r->chunks()->at(3)->code(), 0xff);
    std::string str2 = r->chunks()->at(3)->_raw_body();
    BOOST_CHECK_EQUAL(str2, std::string(""));

    delete r;
}
