package spec::perl::TestBufferedStruct;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use BufferedStruct;

sub test_buffered_struct: Test(7) {
    my $r = BufferedStruct->from_file('src/buffered_struct.bin');

    is($r->len1(), 0x10, 'Equals');
    is($r->block1()->number1(), 0x42, 'Equals');
    is($r->block1()->number2(), 0x43, 'Equals');
    is($r->len2(), 0x8, 'Equals');
    is($r->block2()->number1(), 0x44, 'Equals');
    is($r->block2()->number2(), 0x45, 'Equals');
    is($r->finisher(), 0xee, 'Equals');
}

Test::Class->runtests;
