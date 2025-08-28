package spec::perl::TestDebugEnumName;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use DebugEnumName;

sub test_debug_enum_name: Test(4) {
    my $r = DebugEnumName->from_file('src/fixed_struct.bin');

    # --debug implies --no-auto-read
    $r->_read();

    is($r->one(), $DebugEnumName::TEST_ENUM1_ENUM_VALUE_80, 'Equals');
    is(@{$r->array_of_ints()}[0], $DebugEnumName::TEST_ENUM2_ENUM_VALUE_65, 'Equals');
    is($r->test_type()->field1(), $DebugEnumName::TestSubtype::INNER_ENUM1_ENUM_VALUE_67, 'Equals');
    is($r->test_type()->instance_field(), $DebugEnumName::TestSubtype::INNER_ENUM2_ENUM_VALUE_11, 'Equals');
}

Test::Class->runtests;
