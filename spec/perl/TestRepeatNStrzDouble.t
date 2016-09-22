package spec::perl::TestRepeatNStrzDouble;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use RepeatNStrzDouble;

sub test_repeat_n_strz_double: Test(3) {
    my $r = RepeatNStrzDouble->from_file('src/repeat_n_strz.bin');

    is($r->qty(), 2, 'Equals');

    my @exp1 = ('foo');
    is_deeply($r->lines1(), \@exp1, 'Equals');
    my @exp2 = ('bar');
    is_deeply($r->lines2(), \@exp2, 'Equals');
}

Test::Class->runtests;
