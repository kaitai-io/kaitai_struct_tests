<?php
namespace Kaitai\Struct\Tests;

class HelloWorldTest extends TestCase {
    public function testHelloWorld() {
        $r = HelloWorld::fromFile(self::SRC_DIR_PATH . '/fixed_struct.bin');
        // Access property through direct call
        $this->assertEquals(0x50, $r->one());
    }
}
