<?php
namespace Kaitai\Struct\Tests;

class DebugArrayUserEofExceptionTest extends TestCase {
    public function testDebugArrayUserEofException() {
        $r = DebugArrayUserEofException::fromFile(self::SRC_DIR_PATH . '/nav_parent_codes.bin');

        try {
            // --debug implies --no-auto-read
            $r->_read();
            $this->fail("An exception was expected, but none thrown");
        } catch (\Kaitai\Struct\Error\EndOfStreamError $e) {
            // OK
        }

        $this->assertSame(3, $r->oneCat()->meow());
        $this->assertSame(73, $r->oneCat()->chirp());
        $this->assertSame(3, count($r->arrayOfCats()));
        $this->assertSame(49, $r->arrayOfCats()[0]->meow());
        $this->assertSame(50, $r->arrayOfCats()[0]->chirp());
        $this->assertSame(51, $r->arrayOfCats()[1]->meow());
        $this->assertSame(66, $r->arrayOfCats()[1]->chirp());
        $this->assertSame(98, $r->arrayOfCats()[2]->meow());
    }
}
