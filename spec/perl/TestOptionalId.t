package spec::perl::TestOptionalId;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use OptionalId;

sub test_optional_id: Test(3) {
    my $r = OptionalId->from_file('src/fixed_struct.bin');


    is($r->_unnamed0(), 80, 'Equals');
    is($r->_unnamed1(), 65, 'Equals');
    is($r->_unnamed2(), pack('C*', (67, 75, 45, 49, 255)), 'Equals');
}

Test::Class->runtests;
