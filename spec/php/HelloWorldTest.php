<?php

namespace Kaitai\Tests;

class HelloWorldTest extends \PHPUnit_Framework_TestCase {
    public function test() {
        $r = HelloWorld::fromFile('src/fixed_struct.bin');
        $this->assertEquals($r->one, 0x50);
    }
}
