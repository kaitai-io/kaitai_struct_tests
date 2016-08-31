package spec::perl::TestTypeIntUnaryOp;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use TypeIntUnaryOp;

sub test_type_int_unary_op: Test {
    my $r = TypeIntUnaryOp->from_file('src/fixed_struct.bin');

    is($r->{value_s2}, 0x4150, 'Equals');
    is($r->{value_s8}, 0x4150ffff312d4b43, 'Equals');
    is($r->unary_s2(), -0x4150, 'Equals');
    is($r->unary_s8(), -0x4150ffff312d4b43, 'Equals');
}

Test::Class->runtests;
