<?php
require __DIR__ . '/../../../runtime/php/lib/Kaitai/Struct/Struct.php';
require __DIR__ . '/../../../runtime/php/lib/Kaitai/Struct/Stream.php';

require __DIR__ . '/extra/CustomFx.php';

spl_autoload_register(function ($class) {
    $kt = "Kaitai\\Struct\\Tests\\";
    if (substr($class, 0, strlen($kt)) === $kt) {
        $testName = substr($class, strlen($kt));
        if ($testName === 'TestCase') {
            require __DIR__ . '/TestCase.php';
        } else {
            $fn = __DIR__ . "/../../compiled/php/$testName.php";
            if (file_exists($fn)) {
                require $fn;
            } else {
                trigger_error("attempt to autoload \"$class\" failed", E_USER_WARNING);
            }
        }
    }
});
