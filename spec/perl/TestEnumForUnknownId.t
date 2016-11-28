package spec::perl::TestEnumForUnknownId;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use EnumForUnknownId;

sub test_enum_for_unknown_id: Test(1) {
    my $r = EnumForUnknownId->from_file('src/fixed_struct.bin');

    is($r->one(), 80, 'Equals');
}

Test::Class->runtests;
