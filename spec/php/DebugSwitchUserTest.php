<?php
// runs in debug mode, so the _read() needs to be called manually

namespace Kaitai\Struct\Tests;

class DebugSwitchUserTest extends TestCase {
    public function testDebugSwitchUser() {
        $r = DebugSwitchUser::fromFile(self::SRC_DIR_PATH . '/nav_parent_switch.bin');
        $r->_read();

        $this->assertSame(1, $r->code());
        $this->assertSame(-190, $r->data()->val());
    }
}
