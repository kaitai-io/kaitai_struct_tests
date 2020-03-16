<?php
namespace Kaitai\Struct\Tests;

class SwitchCastTest extends TestCase {
    public function testSwitchCast() {
        $r = SwitchCast::fromFile(self::SRC_DIR_PATH . "/switch_opcodes.bin");

        $this->assertSame('foobar', $r->firstObj->value);
        $this->assertSame(0x42, $r->secondVal);
        # unable to test "err_cast" here
    }
}
