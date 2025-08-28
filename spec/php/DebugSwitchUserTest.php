<?php
namespace Kaitai\Struct\Tests;

class DebugSwitchUserTest extends TestCase {
    public function testDebugSwitchUser() {
        $r = DebugSwitchUser::fromFile(self::SRC_DIR_PATH . '/nav_parent_switch.bin');

        // --debug implies --no-auto-read
        $r->_read();

        $this->assertSame(1, $r->code());
        $this->assertSame(-190, $r->data()->val());
    }
}
