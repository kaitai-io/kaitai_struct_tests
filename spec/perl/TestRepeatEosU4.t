package spec::perl::TestRepeatEosU4;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use RepeatEosU4;

sub test_repeat_eos_u4: Test(1) {
    my $r = RepeatEosU4->from_file('src/repeat_eos_struct.bin');

    my @exp = (0, 0x42, 0x42, 0x815);
    is_deeply($r->numbers(), \@exp, 'Equals');
}

Test::Class->runtests;
