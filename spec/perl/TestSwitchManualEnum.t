package spec::perl::TestSwitchManualEnum;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use SwitchManualEnum;

sub test_switch_manual_enum: Test(9) {
    my $r = SwitchManualEnum->from_file('src/switch_opcodes.bin');

    is($r->opcodes()->size(), 4, 'Equals');

    is($r->opcodes[0]()->code(), SwitchManualEnum->Opcode()->Code()->strval(), 'Equals');
    is($r->opcodes[0]()->body()->value(), 'foobar', 'Equals');

    is($r->opcodes[1]()->code(), SwitchManualEnum->Opcode()->Code()->intval(), 'Equals');
    is($r->opcodes[1]()->body()->value(), 0x42, 'Equals');

    is($r->opcodes[2]()->code(), SwitchManualEnum->Opcode()->Code()->intval(), 'Equals');
    is($r->opcodes[2]()->body()->value(), 0x37, 'Equals');

    is($r->opcodes[3]()->code(), SwitchManualEnum->Opcode()->Code()->strval(), 'Equals');
    is($r->opcodes[3]()->body()->value(), '', 'Equals');
}

Test::Class->runtests;
