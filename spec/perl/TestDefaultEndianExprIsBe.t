package spec::perl::TestDefaultEndianExprIsBe;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use DefaultEndianExprIsBe;

sub test_default_endian_expr_is_be: Test(18) {
    my $r = DefaultEndianExprIsBe->from_file('src/endian_expr.bin');

    # LE
    is($r->docs()->[0]->indicator(), 'II', 'Equals');
    is($r->docs()->[0]->main()->some_int(), 0x42, 'Equals');
    is($r->docs()->[0]->main()->some_int_be(), 0x42, 'Equals');
    is($r->docs()->[0]->main()->some_int_le(), 0x42, 'Equals');

    is($r->docs()->[0]->main()->inst_int(), 0x42, 'Equals');
    is($r->docs()->[0]->main()->inst_sub()->foo(), 0x42, 'Equals');

    # BE
    is($r->docs()->[1]->indicator(), 'MM', 'Equals');
    is($r->docs()->[1]->main()->some_int(), 0x42, 'Equals');
    is($r->docs()->[1]->main()->some_int_be(), 0x42, 'Equals');
    is($r->docs()->[1]->main()->some_int_le(), 0x42, 'Equals');

    is($r->docs()->[1]->main()->inst_int(), 0x42000000, 'Equals');
    is($r->docs()->[1]->main()->inst_sub()->foo(), 0x42000000, 'Equals');

    # Weird => LE
    is($r->docs()->[2]->indicator(), 'XX', 'Equals');
    is($r->docs()->[2]->main()->some_int(), 0x42000000, 'Equals');
    is($r->docs()->[2]->main()->some_int_be(), 0x42, 'Equals');
    is($r->docs()->[2]->main()->some_int_le(), 0x42, 'Equals');

    is($r->docs()->[2]->main()->inst_int(), 0x42, 'Equals');
    is($r->docs()->[2]->main()->inst_sub()->foo(), 0x42, 'Equals');
}

Test::Class->runtests;
