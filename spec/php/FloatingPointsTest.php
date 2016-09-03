<?php
namespace Kaitai\Struct\Tests;

class FloatingPointsTest extends TestCase {
    public function testFloatingPoints() {
        $fp = FloatingPoints::fromFile(self::SRC_DIR_PATH . "/floating_points.bin");

        $delta = 1e-6;

        $this->assertEquals(0.5, $fp->singleValue());
        $this->assertEquals(0.5, $fp->singleValueBe());

        $this->assertEquals(0.25, $fp->doubleValue());
        $this->assertEquals(0.25, $fp->doubleValueBe());

        $this->assertEquals(1.2345, $fp->approximateValue(), '', $delta);
        $this->assertEquals(1.5, $fp->singleValuePlusInt(), '', $delta);
        $this->assertEquals(1.0, $fp->singleValuePlusFloat(), '', $delta);
        $this->assertEquals(0.3, $fp->doubleValuePlusFloat(), '', $delta);
    }
}
