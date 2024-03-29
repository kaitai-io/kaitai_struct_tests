# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestBitsByteAligned;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use BitsByteAligned;

sub test_bits_byte_aligned: Test(9) {
    my $r = BitsByteAligned->from_file('src/fixed_struct.bin');

    is($r->one(), 20, 'Equals');
    is($r->byte_1(), 65, 'Equals');
    is($r->two(), 2, 'Equals');
    cmp_ok($r->three(), '==', 0, 'Equals');
    is($r->byte_2(), 75, 'Equals');
    is($r->four(), 2892, 'Equals');
    is($r->byte_3(), pack('C*', (255)), 'Equals');
    is($r->full_byte(), 255, 'Equals');
    is($r->byte_4(), 80, 'Equals');
}

Test::Class->runtests;
