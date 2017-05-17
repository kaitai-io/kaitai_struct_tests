package spec::perl::TestSwitchBytearray;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use SwitchBytearray;

sub test_switch_bytearray: Test(9) {
    my $r = SwitchBytearray->from_file('src/switch_opcodes.bin');

    is(scalar(@{$r->opcodes()}), 4, 'Equals');

    is($r->opcodes()->[0]->code(), "S", 'Equals');
    is($r->opcodes()->[0]->body()->value(), 'foobar', 'Equals');

    is($r->opcodes()->[1]->code(), "I", 'Equals');
    is($r->opcodes()->[1]->body()->value(), 0x42, 'Equals');

    is($r->opcodes()->[2]->code(), "I", 'Equals');
    is($r->opcodes()->[2]->body()->value(), 0x37, 'Equals');

    is($r->opcodes()->[3]->code(), "S", 'Equals');
    is($r->opcodes()->[3]->body()->value(), '', 'Equals');
}

Test::Class->runtests;
