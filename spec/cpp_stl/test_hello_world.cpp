#define BOOST_TEST_MAIN
#if !defined( WIN32 )
    #define BOOST_TEST_DYN_LINK
#endif
#include <boost/test/unit_test.hpp>

#include <HelloWorld.h>

// #define BOOST_TEST_MODULE HelloWorld

#include <iostream>
#include <fstream>

BOOST_AUTO_TEST_CASE(my_test) {
//    HelloWorld* r = HelloWorld::fromFile("src/fixed_struct.bin");

    std::ifstream ifs("src/fixed_struct.bin", std::ifstream::binary);
    KaitaiStream ks(&ifs);
    HelloWorld* r = new HelloWorld(&ks);

    BOOST_CHECK_EQUAL(r->one(), 0x50);

    delete r;
}
