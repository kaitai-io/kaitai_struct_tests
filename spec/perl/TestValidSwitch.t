# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestValidSwitch;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ValidSwitch;

sub test_valid_switch: Test(0) {
    my $r = ValidSwitch->from_file('src/fixed_struct.bin');

}

Test::Class->runtests;
