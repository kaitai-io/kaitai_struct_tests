package spec::perl::TestRepeatUntilSized;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use RepeatUntilSized;

sub test_repeat_until_sized: Test(7) {
    my $r = RepeatUntilSized->from_file('src/repeat_until_process.bin');

    is(scalar(@{$r->records()}), 3, 'Equals');

    is($r->records()->[0]->marker(), 0xe8, 'Equals');
    is($r->records()->[0]->body(), 0xaaaaaaba, 'Equals');

    is($r->records()->[1]->marker(), 0xfa, 'Equals');
    is($r->records()->[1]->body(), 0xaaaab89e, 'Equals');

    is($r->records()->[2]->marker(), 0xaa, 'Equals');
    is($r->records()->[2]->body(), 0x55555555, 'Equals');
}

Test::Class->runtests;
