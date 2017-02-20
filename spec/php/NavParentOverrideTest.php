<?php
namespace Kaitai\Struct\Tests;

class NavParentOverrideTest extends TestCase {
    public function testNavParentOverride() {
        $r = NavParentOverride::fromFile(self::SRC_DIR_PATH . "/nav_parent_codes.bin");

        $this->assertEquals(3, $r->childSize);
        $this->assertEquals("I12", $r->child1->data);
        $this->assertEquals("3Bb", $r->mediator2->child2->data);
    }
}
