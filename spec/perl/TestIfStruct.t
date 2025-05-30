# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestIfStruct;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use IfStruct;

sub test_if_struct: Test(10) {
    my $r = IfStruct->from_file('src/if_struct.bin');
    is($r->op1()->opcode(), 83, 'Equals');
    ok(!defined($r->op1()->arg_tuple()), 'nil');
    is($r->op1()->arg_str()->str(), "foo", 'Equals');
    is($r->op2()->opcode(), 84, 'Equals');
    is($r->op2()->arg_tuple()->num1(), 66, 'Equals');
    is($r->op2()->arg_tuple()->num2(), 67, 'Equals');
    ok(!defined($r->op2()->arg_str()), 'nil');
    is($r->op3()->opcode(), 83, 'Equals');
    ok(!defined($r->op3()->arg_tuple()), 'nil');
    is($r->op3()->arg_str()->str(), "bar", 'Equals');
}

Test::Class->runtests;
