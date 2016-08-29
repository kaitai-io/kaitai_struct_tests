NUMBERNUMBERNUMBERpackage spec::perl::TestRepeatNStruct;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use RepeatNStruct;

sub test_repeat_n_struct: Test {
    my $r = RepeatNStruct->from_file("src/repeat_n_struct.bin");

    is($len(r->{chunks})}, 2}, 'Equals');
    is($r->{chunks[0]}->{offset}, 0x10, 'Equals');
    is($r->{chunks[0]}->{len}, 0x2078, 'Equals');
    is($r->{chunks[1]}->{offset}, 0x2088, 'Equals');
    is($r->{chunks[1]}->{len}, 0xf}, 'Equals');
}

Test::Class->runtests;
