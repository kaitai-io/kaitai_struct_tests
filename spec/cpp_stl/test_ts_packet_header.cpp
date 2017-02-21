#include <boost/test/unit_test.hpp>

#include <ts_packet_header.h>

#include <iostream>
#include <fstream>
#include <vector>

/**
 * format
 * |7,6,5,4,2,1,0|7,6,5,4,3,2,1,0|7,6,5,4,3,2,1,0|7,6,5,4,3,2,1,0|
 * | sync_byte   | , , ,<-------pid------------->|   ,afc,<-cc-->|
 */
BOOST_AUTO_TEST_CASE(test_ts_packet_header) {
    std::ifstream ifs("src/ts_packet.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    ts_packet_header_t* tspr = new ts_packet_header_t(&ks);
    BOOST_CHECK_EQUAL(tspr != 0, true);
    if (tspr != 0) {
        BOOST_CHECK_EQUAL(tspr->sync_byte(), 0x47);
        BOOST_CHECK_EQUAL(tspr->transport_error_indicator(), 0);
        BOOST_CHECK_EQUAL(tspr->payload_unit_start_indicator(), 0);
        BOOST_CHECK_EQUAL(tspr->transport_priority(), 1);
        BOOST_CHECK_EQUAL(tspr->pid(), 33);
        BOOST_CHECK_EQUAL(tspr->transport_scrambling_control(), 0);
        BOOST_CHECK_EQUAL(tspr->adaptation_field_control(), 1);
    }
    delete tspr;
}
