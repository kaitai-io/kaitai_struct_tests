package spec::perl::TestRepeatUntilS4;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use RepeatUntilS4;

sub test_repeat_until_s4: Test(2) {
    my $r = RepeatUntilS4->from_file("src/repeat_until_s4.bin");

    my @exp = (0x42, 0x1337, -251658241, -1);
    is_deeply($r->{entries}, \@exp, 'Equals');
    is($r->{afterall}, "foobar", 'Equals');
}

Test::Class->runtests;
