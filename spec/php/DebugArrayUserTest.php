<?php
namespace Kaitai\Struct\Tests;

class DebugArrayUserTest extends TestCase {
    public function testDebugArrayUser() {
        $r = DebugArrayUser::fromFile(self::SRC_DIR_PATH . '/fixed_struct.bin');
        $r->_read();

        $this->assertSame(0x50, $r->oneCat()->meow());
        $this->assertSame(0x41, $r->arrayOfCats()[0]->meow());
        $this->assertSame(0x43, $r->arrayOfCats()[1]->meow());
        $this->assertSame(0x4b, $r->arrayOfCats()[2]->meow());
    }
}
