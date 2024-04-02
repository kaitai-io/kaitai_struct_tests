#include <boost/test/unit_test.hpp>
#include "str_encodings_escaping_enc.h"
#include <iostream>
#include <fstream>
#include <vector>
#include "kaitai/exceptions.h"

static bool check_str1_message(const kaitai::unknown_encoding& e) {
    BOOST_CHECK_EQUAL(e.what(), std::string("bytes_to_str error: unknown encoding: `ASCII\\\\x`"));
    return true;
}

static bool check_str2_message(const kaitai::unknown_encoding& e) {
    BOOST_CHECK_EQUAL(e.what(), std::string("bytes_to_str error: unknown encoding: `UTF-8\\'x`"));
    return true;
}

static bool check_str3_message(const kaitai::unknown_encoding& e) {
    BOOST_CHECK_EQUAL(e.what(), std::string("bytes_to_str error: unknown encoding: `SJIS\\\"x`"));
    return true;
}

static bool check_str4_message(const kaitai::unknown_encoding& e) {
    BOOST_CHECK_EQUAL(e.what(), std::string("bytes_to_str error: unknown encoding: `IBM437\\nx`"));
    return true;
}

BOOST_AUTO_TEST_CASE(test_str_encodings_escaping_enc) {
    std::ifstream ifs("src/str_encodings.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    str_encodings_escaping_enc_t* r = new str_encodings_escaping_enc_t(&ks);

    BOOST_CHECK_EXCEPTION(
        r->str1()->v(),
        kaitai::unknown_encoding,
        check_str1_message
    );
    BOOST_CHECK_EXCEPTION(
        r->str2()->v(),
        kaitai::unknown_encoding,
        check_str2_message
    );
    BOOST_CHECK_EXCEPTION(
        r->str3()->v(),
        kaitai::unknown_encoding,
        check_str3_message
    );
    BOOST_CHECK_EXCEPTION(
        r->str4()->v(),
        kaitai::unknown_encoding,
        check_str4_message
    );

    delete r;
}
