<?php
namespace Kaitai\Struct\Tests;

class DebugArrayUserTest extends TestCase {
    public function testDebugArrayUser() {
        $r = DebugArrayUser::fromFile(self::SRC_DIR_PATH . '/fixed_struct.bin');

        // --debug implies --no-auto-read
        $r->_read();

        $this->assertSame(80, $r->oneCat()->meow());
        $this->assertSame(3, count($r->arrayOfCats()));
        $this->assertSame(65, $r->arrayOfCats()[0]->meow());
        $this->assertSame(67, $r->arrayOfCats()[1]->meow());
        $this->assertSame(75, $r->arrayOfCats()[2]->meow());
    }
}
