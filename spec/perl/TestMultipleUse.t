package spec::perl::TestMultipleUse;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use MultipleUse;

sub test_multiple_use: Test(2) {
    my $r = MultipleUse->from_file('src/position_abs.bin');

    is($r->{t1}->{first_use}->{value}, 0x20, 'Equals');
    is($r->{t2}->second_use()->{value}, 0x20, 'Equals');
}

Test::Class->runtests;
