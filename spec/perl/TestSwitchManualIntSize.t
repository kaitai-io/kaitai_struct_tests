package spec::perl::TestSwitchManualIntSize;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use SwitchManualIntSize;

sub test_switch_manual_int_size: Test(10) {
    my $r = SwitchManualIntSize->from_file('src/switch_tlv.bin');

    is(scalar(@{$r->chunks()}), 4, 'Equals');

    is($r->chunks()->[0]->code(), 0x11, 'Equals');
    my $meta = $r->chunks()->[0]->body();
    is($meta->title(), 'Stuff', 'Equals');
    is($meta->author(), 'Me', 'Equals');

    is($r->chunks()->[1]->code(), 0x22, 'Equals');
    is_deeply($r->chunks()->[1]->body()->entries(), ['AAAA', 'BBBB', 'CCCC'], 'Equals');

    is($r->chunks()->[2]->code(), 0x33, 'Equals');
    is_deeply($r->chunks()->[2]->body(), pack('C*', (0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80)), 'Equals');

    is($r->chunks()->[3]->code(), 0xff, 'Equals');
    is($r->chunks()->[3]->body(), "", 'Equals');
}

Test::Class->runtests;
