package spec::perl::TestDebugEnumName;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use DebugEnumName;

sub test_debug_enum_name: Test(0) {
    my $r = DebugEnumName->from_file('src/fixed_struct.bin');

    # this test is meaningful only for languages that have --debug and do
    # not save enum type info
}

Test::Class->runtests;
