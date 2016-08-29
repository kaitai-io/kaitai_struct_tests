package spec::perl::TestRepeatUntilComplex;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use RepeatUntilComplex;

sub test_repeat_until_complex: Test {
    my $r = RepeatUntilComplex->from_file("src/repeat_until_complex.bin");

    is($len(r->{first})}, 3}, 'Equals');
    is($r->{first[0]}->{count}, 4}, 'Equals');
    is($r->{first[0]}->{values}, [1, 2, 3, 4]}, 'Equals');
    is($r->{first[1]}->{count}, 2}, 'Equals');
    is($r->{first[1]}->{values}, [1, 2]}, 'Equals');
    is($r->{first[2]}->{count}, 0}, 'Equals');
    is($r->{first[2]}->{values}, []}, 'Equals');

    is($len(r->{second})}, 4}, 'Equals');
    is($r->{second[0]}->{count}, 6}, 'Equals');
    is($r->{second[0]}->{values}, [1, 2, 3, 4, 5, 6]}, 'Equals');
    is($r->{second[1]}->{count}, 3}, 'Equals');
    is($r->{second[1]}->{values}, [1, 2, 3]}, 'Equals');
    is($r->{second[2]}->{count}, 4}, 'Equals');
    is($r->{second[2]}->{values}, [1, 2, 3, 4]}, 'Equals');
    is($r->{second[3]}->{count}, 0}, 'Equals');
    is($r->{second[3]}->{values}, []}, 'Equals');
        
    is($r->{third}, [102, 111, 111, 98, 97, 114, 0]}, 'Equals');
}

Test::Class->runtests;
