<?php
namespace Kaitai\Struct\Tests;

class RepeatUntilComplexTest extends TestCase {
    public function testRepeatUntilComplex() {
        $r = RepeatUntilComplex::fromFile(self::SRC_DIR_PATH . "/repeat_until_complex.bin");

        $this->assertEquals(3, count($r->first));
        $this->assertEquals(4, $r->first[0]->count);
        $this->assertEquals([1, 2, 3, 4], $r->first[0]->values);
        $this->assertEquals(2, $r->first[1]->count);
        $this->assertEquals([1, 2], $r->first[1]->values);
        $this->assertEquals(0, $r->first[2]->count);
        $this->assertEquals([], $r->first[2]->values);

        $this->assertEquals(4, count($r->second));
        $this->assertEquals(6, $r->second[0]->count);
        $this->assertEquals([1, 2, 3, 4, 5, 6], $r->second[0]->values);
        $this->assertEquals(3, $r->second[1]->count);
        $this->assertEquals([1, 2, 3], $r->second[1]->values);
        $this->assertEquals(4, $r->second[2]->count);
        $this->assertEquals([1, 2, 3, 4], $r->second[2]->values);
        $this->assertEquals(0, $r->second[3]->count);
        $this->assertEquals([], $r->second[3]->values);
        
        $this->assertEquals([102, 111, 111, 98, 97, 114, 0], $r->third);
    }
}
