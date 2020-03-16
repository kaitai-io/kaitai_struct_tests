<?php
namespace Kaitai\Struct\Tests;

class EnumLongRangeSTest extends TestCase {
    public function testEnumLongRangeS() {
        $r = EnumLongRangeS::fromFile(self::SRC_DIR_PATH . '/enum_long_range_s.bin');

        $this->assertSame(intval(\Kaitai\Struct\Tests\EnumLongRangeS\Constants::LONG_MIN), $r->f1()); // PHP bug: PHP_INT_MIN is float (https://bugs.php.net/bug.php?id=76385)
        $this->assertSame(\Kaitai\Struct\Tests\EnumLongRangeS\Constants::INT_BELOW_MIN, $r->f2());
        $this->assertSame(\Kaitai\Struct\Tests\EnumLongRangeS\Constants::INT_MIN, $r->f3());
        $this->assertSame(\Kaitai\Struct\Tests\EnumLongRangeS\Constants::ZERO, $r->f4());
        $this->assertSame(\Kaitai\Struct\Tests\EnumLongRangeS\Constants::INT_MAX, $r->f5());
        $this->assertSame(\Kaitai\Struct\Tests\EnumLongRangeS\Constants::INT_OVER_MAX, $r->f6());
        $this->assertSame(\Kaitai\Struct\Tests\EnumLongRangeS\Constants::LONG_MAX, $r->f7());
    }
}
