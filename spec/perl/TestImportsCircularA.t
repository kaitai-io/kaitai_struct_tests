package spec::perl::TestImportsCircularA;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ImportsCircularA;

sub test_imports_circular_a: Test(5) {
    my $r = ImportsCircularA->from_file('src/fixed_struct.bin');

    is($r->code(), 0x50, 'Equals');
    is($r->two()->initial(), 0x41, 'Equals');
    is($r->two()->back_ref()->code(), 0x43, 'Equals');
    is($r->two()->back_ref()->two()->initial(), 0x4b, 'Equals');
    is($r->two()->back_ref()->two()->back_ref(), undef, 'Equals');
}

Test::Class->runtests;
