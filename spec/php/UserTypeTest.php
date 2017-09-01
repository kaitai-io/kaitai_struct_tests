<?php
namespace Kaitai\Struct\Tests;

class UserTypeTest extends TestCase {
    public function testUserType() {
        $r = UserType::fromFile(self::SRC_DIR_PATH . "/repeat_until_s4.bin");

        $this->assertEquals(0x42, $r->one->width);
        $this->assertEquals(0x1337, $r->one->height);
    }
}
