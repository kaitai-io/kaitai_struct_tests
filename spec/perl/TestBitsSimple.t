package spec::perl::TestBitsSimple;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use BitsSimple;

sub test_bits_simple: Test(10) {
    my $r = BitsSimple->from_file('src/fixed_struct.bin');

    # 50 41
    is($r->byte_1(), 0x50, 'Equals');
    is($r->byte_2(), 0x41, 'Equals');

    # 43 (1 + 3 + 4) = 0|100|0011
    is($r->bits_a(), 0, 'Equals');
    is($r->bits_b(), 0b100, 'Equals');
    is($r->bits_c(), 0b0011, 'Equals');

    # 4B 2D 31 (10 + 3 + 11) = 01001011 00|101|101 00110001
    is($r->large_bits_1(), 0b0100101100, 'Equals');
    is($r->spacer(), 0b101, 'Equals');
    is($r->large_bits_2(), 0b10100110001, 'Equals');

    # FF FF
    is($r->normal_s2(), -1, 'Equals');

    # 50 41 43
    is($r->byte_8_9_10(), 0x504143, 'Equals');
}

Test::Class->runtests;
