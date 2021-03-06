# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestBitsSignedB64Le;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use BitsSignedB64Le;

sub test_bits_signed_b64_le: Test(4) {
    my $r = BitsSignedB64Le->from_file('src/bits_signed_b64_le.bin');


    is($r->a_num(), 0, 'Equals');
    is($r->a_bit(), 1, 'Equals');
    is($r->b_num(), 9223372036854775807, 'Equals');
    is($r->b_bit(), 0, 'Equals');
}

Test::Class->runtests;
