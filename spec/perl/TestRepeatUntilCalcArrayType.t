# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestRepeatUntilCalcArrayType;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use RepeatUntilCalcArrayType;

sub test_repeat_until_calc_array_type: Test(7) {
    my $r = RepeatUntilCalcArrayType->from_file('src/repeat_until_process.bin');

    is(scalar(@{$r->records()}), 3, 'Equals');
    is(@{$r->records()}[0]->marker(), 232, 'Equals');
    is(@{$r->records()}[0]->body(), 2863311546, 'Equals');
    is(@{$r->records()}[1]->marker(), 250, 'Equals');
    is(@{$r->records()}[1]->body(), 2863315102, 'Equals');
    is(@{$r->records()}[2]->marker(), 170, 'Equals');
    is(@{$r->records()}[2]->body(), 1431655765, 'Equals');
}

Test::Class->runtests;
