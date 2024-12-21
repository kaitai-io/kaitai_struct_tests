package spec::perl::TestDebug0;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use Debug0;

sub test_debug_0: Test(6) {
    my $r = Debug0->from_file('src/fixed_struct.bin');

    # --debug implies --no-auto-read
    $r->_read();

    is($r->one(), 80, 'Equals');
    is(scalar(@{$r->array_of_ints()}), 3, 'Equals');
    is(@{$r->array_of_ints()}[0], 65, 'Equals');
    is(@{$r->array_of_ints()}[1], 67, 'Equals');
    is(@{$r->array_of_ints()}[2], 75, 'Equals');
    is($r->_unnamed2(), 45, 'Equals');

    # FIXME: also test --read-pos once it is implemented
}

Test::Class->runtests;
