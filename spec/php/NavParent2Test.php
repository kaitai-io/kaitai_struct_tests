<?php
namespace Kaitai\Struct\Tests;

class NavParent2Test extends TestCase {
    public function testNavParent2() {
        $r = NavParent2::fromFile(self::SRC_DIR_PATH . "/nav_parent2.bin");

        $this->assertEquals(8, $r->ofsTags);
        $this->assertEquals(2, $r->numTags);

        $this->assertEquals('RAHC', $r->tags[0]->name);
        $this->assertEquals(0x20, $r->tags[0]->ofs);
        $this->assertEquals(3, $r->tags[0]->numItems);
        $this->assertEquals('foo', $r->tags[0]->tagContent->content);

        $this->assertEquals('RAHC', $r->tags[1]->name);
        $this->assertEquals(0x23, $r->tags[1]->ofs);
        $this->assertEquals(6, $r->tags[1]->numItems);
        $this->assertEquals('barbaz', $r->tags[1]->tagContent->content);
    }
}
