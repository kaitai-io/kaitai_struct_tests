package spec::perl::TestIfStruct;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use IfStruct;

sub test_if_struct: Test(7) {
    my $r = IfStruct->from_file('src/if_struct.bin');

    is($r->{op1}->{opcode}, 0x53, 'Equals');
    is($r->{op1}->{arg_str}->{str}, 'foo', 'Equals');

    is($r->{op2}->{opcode}, 0x54, 'Equals');
    is($r->{op2}->{arg_tuple}->{num1}, 0x42, 'Equals');
    is($r->{op2}->{arg_tuple}->{num2}, 0x43, 'Equals');

    is($r->{op3}->{opcode}, 0x53, 'Equals');
    is($r->{op3}->{arg_str}->{str}, 'bar', 'Equals');
}

Test::Class->runtests;
