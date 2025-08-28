<?php
namespace Kaitai\Struct\Tests;

class DebugArrayUserCurrentExcludedTest extends TestCase {
    public function testDebugArrayUserCurrentExcluded() {
        $r = DebugArrayUserCurrentExcluded::fromFile(self::SRC_DIR_PATH . '/term_strz.bin');

        // --debug implies --no-auto-read
        $r->_read();

        $this->assertSame("\x66\x6F\x6F", $r->arrayOfCats()[0]->meow());
        $this->assertSame("\x7C\x62", $r->arrayOfCats()[1]->meow());
        $this->assertSame("\x61", $r->arrayOfCats()[2]->meow());
    }
}
