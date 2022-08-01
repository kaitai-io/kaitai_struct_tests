<?php
namespace Kaitai\Struct\Tests;

class MyCustomFx {
    public function __construct($key, $flag, $someBytes) {
        $this->key = $flag ? $key : -$key;
    }

    public function decode($src) {
        $dst = '';
        for ($i = 0, $n = strlen($src); $i < $n; $i++) {
            $dst .= chr(ord($src[$i]) + $this->key);
        }
        return $dst;
    }
}

class CustomFxNoArgs {
    public function __construct() {
    }

    public function decode($src) {
        return "_" . $src . "_";
    }
}

namespace Nested\Deeply;

class CustomFx {
    public function __construct(int $x) {
    }

    public function decode($src) {
        return "_" . $src . "_";
    }
}
