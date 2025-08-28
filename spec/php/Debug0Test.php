<?php
namespace Kaitai\Struct\Tests;

class Debug0Test extends TestCase {
    public function testDebug0() {
        $r = Debug0::fromFile(self::SRC_DIR_PATH . '/fixed_struct.bin');

        // --debug implies --no-auto-read
        $r->_read();

        $this->assertSame(80, $r->one());
        $this->assertSame(3, count($r->arrayOfInts()));
        $this->assertSame(65, $r->arrayOfInts()[0]);
        $this->assertSame(67, $r->arrayOfInts()[1]);
        $this->assertSame(75, $r->arrayOfInts()[2]);
        $this->assertSame(45, $r->_unnamed2());

        // FIXME: also test --read-pos once it is implemented
    }
}
