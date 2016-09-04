package spec::perl::TestRepeatEosStruct;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use RepeatEosStruct;

sub test_repeat_eos_struct: Test(5) {
    my $r = RepeatEosStruct->from_file('src/repeat_eos_struct.bin');

    is(scalar @{ $r->chunks() }, 2, 'Equals');
    is(@{ $r->chunks() }[0]->offset(), 0, 'Equals');
    is(@{ $r->chunks() }[0]->len(), 0x42, 'Equals');
    is(@{ $r->chunks() }[1]->offset(), 0x42, 'Equals');
    is(@{ $r->chunks() }[1]->len(), 0x815, 'Equals');
}

Test::Class->runtests;
