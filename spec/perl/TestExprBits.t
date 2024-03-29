# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestExprBits;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ExprBits;

sub test_expr_bits: Test(10) {
    my $r = ExprBits->from_file('src/switch_opcodes.bin');

    is($r->a(), 2, 'Equals');
    is($r->enum_seq(), $ExprBits::ITEMS_FOO, 'Equals');
    is($r->byte_size(), pack('C*', (102, 111)), 'Equals');
    is(scalar(@{$r->repeat_expr()}), 2, 'Equals');
    is(@{$r->repeat_expr()}[0], 111, 'Equals');
    is(@{$r->repeat_expr()}[1], 98, 'Equals');
    is($r->switch_on_type(), 97, 'Equals');
    is($r->switch_on_endian()->foo(), 29184, 'Equals');
    is($r->enum_inst(), $ExprBits::ITEMS_BAR, 'Equals');
    is($r->inst_pos(), 111, 'Equals');
}

Test::Class->runtests;
