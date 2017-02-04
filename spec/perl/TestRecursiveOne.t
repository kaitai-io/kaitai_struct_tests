package spec::perl::TestRecursiveOne;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use RecursiveOne;

sub test_recursive_one: Test(4) {
    my $r = RecursiveOne->from_file('src/fixed_struct.bin');

    is($r->one(), 0x50, 'Equals');
    is($r->next()->one(), 0x41, 'Equals');
    is($r->next()->next()->one(), 0x43, 'Equals');
    is($r->next()->next()->next()->finisher(), 0x2d4b, 'Equals');
}

Test::Class->runtests;
