package spec::perl::TestDebugSwitchUser;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use DebugSwitchUser;

sub test_debug_switch_user: Test(2) {
    my $r = DebugSwitchUser->from_file('src/nav_parent_switch.bin');

    # --debug implies --no-auto-read
    $r->_read();

    is($r->code(), 1, 'Equals');
    is($r->data()->val(), -190, 'Equals');
}

Test::Class->runtests;
