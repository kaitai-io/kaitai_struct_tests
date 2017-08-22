package spec::perl::TestCastToTop;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use CastToTop;

sub test_cast_to_top: Test(3) {
    my $r = CastToTop->from_file('src/fixed_struct.bin');

    is($r->code(), 0x50, 'Equals');
    is($r->header()->code(), 0x41, 'Equals');
    is($r->header_casted()->code(), 0x41, 'Equals');
}

Test::Class->runtests;
