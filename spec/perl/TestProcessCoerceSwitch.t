package spec::perl::TestProcessCoerceSwitch;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ProcessCoerceSwitch;

sub test_process_coerce_switch: Test(3) {
    my $r = ProcessCoerceSwitch->from_file('src/process_coerce_switch.bin');

    is($r->buf_type(), 0, 'Equals');
    is($r->flag(), 0, 'Equals');
    is($r->buf()->bar(), 'AAAA', 'Equals');
}

Test::Class->runtests;
