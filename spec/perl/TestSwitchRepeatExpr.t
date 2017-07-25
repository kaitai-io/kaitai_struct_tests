package spec::perl::TestSwitchRepeatExpr;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use SwitchRepeatExpr;

sub test_switch_repeat_expr: Test(3) {
    my $r = SwitchRepeatExpr->from_file('src/switch_tlv.bin');

    is($r->code(), 0x11, 'Equals');
    is($r->size(), 9, 'Equals');
    is($r->body()->[0]->first(), "Stuff\0Me\0", 'Equals');
}

Test::Class->runtests;
