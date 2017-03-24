package spec::perl::TestNavParentOverride;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use NavParentOverride;

sub test_nav_parent_override: Test(3) {
    my $r = NavParentOverride->from_file('src/nav_parent_codes.bin');

    is($r->child_size(), 3, 'Equals');
    is_deeply($r->child_1()->data(), pack('C*', (73, 49, 50)), 'Equals');
    is_deeply($r->mediator_2()->child_2()->data(), pack('C*', (51, 66, 98)), 'Equals');
}

Test::Class->runtests;
