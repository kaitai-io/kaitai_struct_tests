<?php
namespace Kaitai\Struct\Tests;

class IfInstancesTest extends TestCase {
    public function testIfInstances() {
        $r = IfInstances::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals($r->neverHappens, null);
    }
}
