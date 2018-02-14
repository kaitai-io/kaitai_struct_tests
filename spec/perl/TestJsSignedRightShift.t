package spec::perl::TestJsSignedRightShift;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use JsSignedRightShift;

sub test_js_signed_right_shift: Test(2) {
    my $r = JsSignedRightShift->from_file('src/fixed_struct.bin');

    is($r->should_be_40000000(), 0x40000000, 'Equals');
    is($r->should_be_a00000(), 0xa00000, 'Equals');
}

Test::Class->runtests;
