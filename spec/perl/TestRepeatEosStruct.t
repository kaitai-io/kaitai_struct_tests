package spec::perl::TestRepeatEosStruct;

use base qw(Test::Class);
use Test::More;
use RepeatEosStruct;

sub test_repeat_eos_struct : Test {
    my $r = RepeatEosStruct->from_file('src/repeat_eos_struct.bin');
    is(scalar @{$r->{chunks}}, 2, 'equals');
    is(@{$r->{chunks}}[0]->{offset}, 0, 'equals');
    is(@{$r->{chunks}}[0]->{size}, 0x42, 'equals');
}

Test::Class->runtests;
