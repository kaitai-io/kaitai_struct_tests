package spec::perl::TestOptionalId;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use OptionalId;

sub test_optional_id: Test(3) {
    my $r = OptionalId->from_file('src/fixed_struct.bin');

    is($r->_unnamed0(), 0x50, 'Equals');
    is($r->_unnamed1(), 0x41, 'Equals');
    is_deeply($r->_unnamed2(), pack('C*', (0x43, 0x4b, 0x2d, 0x31, 0xff)), 'Equals');
}

Test::Class->runtests;
