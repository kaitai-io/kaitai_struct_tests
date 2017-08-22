package spec::perl::TestCastToImported;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use CastToImported;

sub test_cast_to_imported: Test(2) {
    my $r = CastToImported->from_file('src/fixed_struct.bin');

    is($r->one()->one(), 0x50, 'Equals');
    is($r->one_casted()->one(), 0x50, 'Equals');
}

Test::Class->runtests;
