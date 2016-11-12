<?php
namespace Kaitai\Struct\Tests;

class NavParentSwitchTest extends TestCase {
    public function testNavParentSwitch() {
        $r = NavParentSwitch::fromFile(self::SRC_DIR_PATH . "/nav_parent_switch.bin");

        $this->assertEquals(1, $r->category);
        $this->assertEquals(0x42, $r->content->foo);
        $this->assertEquals(0xff, $r->content->subelement->bar);
    }
}
