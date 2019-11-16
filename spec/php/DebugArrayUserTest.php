<?php
namespace Kaitai\Struct\Tests;

class DebugArrayUserTest extends TestCase {
    public function testDebugArrayUser() {
        $r = DebugArrayUser::fromFile(self::SRC_DIR_PATH . '/fixed_struct.bin');

        $r->_read();

        $this->assertEquals(0x50, $r->oneCat()->meow());
        $this->assertEquals(0x41, $r->arrayOfCats()[0]->meow());
        $this->assertEquals(0x43, $r->arrayOfCats()[1]->meow());
        $this->assertEquals(0x4b, $r->arrayOfCats()[2]->meow());
    }
}
