package spec::perl::TestEnumIf;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use EnumIf;
use Data::Dumper qw(Dumper);

sub test_enum_0: Test(7) {
    my $r = EnumIf->from_file('src/if_struct.bin');
    
    is($r->{op1}->{opcode}, $EnumIf::OPCODES_A_STRING, 'Equals');
    is($r->{op1}->{arg_str}->{str}, 'foo');

    is($r->{op2}->{opcode}, $EnumIf::OPCODES_A_TUPLE, 'Equals');
    is($r->{op2}->{arg_tuple}->{num1}, 0x42);
    is($r->{op2}->{arg_tuple}->{num2}, 0x43);

    is($r->{op3}->{opcode}, $EnumIf::OPCODES_A_STRING, 'Equals');
    is($r->{op3}->{arg_str}->{str}, 'bar');
}

Test::Class->runtests;
