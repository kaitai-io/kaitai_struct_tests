package spec::perl::TestBitsByteAligned;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use BitsByteAligned;

sub test_bits_byte_aligned: Test(9) {
    my $r = BitsByteAligned->from_file('src/fixed_struct.bin');

    # 50 (6 + 2) = 010100|00
    is($r->one(), 0b010100, 'Equals');
    # 41
    is($r->byte_1(), 0x41, 'Equals');
    # 43 (3 + 1 + 4) = 010|0|0011
    is($r->two(), 0b010, 'Equals');
    is($r->three(), 0, 'Equals');
    # 4B
    is($r->byte_2(), 0x4b, 'Equals');
    # 2D 31 (14 + 2) = 00101101 001100|01
    is($r->four(), 0b00101101_001100, 'Equals');
    # FF
    is_deeply($r->byte_3(), pack('C*', (0xff)), 'Equals');
    # FF
    is($r->full_byte(), 0xff, 'Equals');
    # 50
    is($r->byte_4(), 0x50, 'Equals');
}

Test::Class->runtests;
