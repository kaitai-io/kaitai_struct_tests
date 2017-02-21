package spec::perl::TestSwitchCast;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use SwitchCast;

sub test_switch_cast: Test(2) {
    my $r = SwitchCast->from_file('src/switch_opcodes.bin');

    is($r->first_obj()->value(), 'foobar', 'Equals');
    is($r->second_val(), 0x42, 'Equals');
    # unable to test 'err_cast' here
}

Test::Class->runtests;
