package spec::perl::TestImports0;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use Imports0;

sub test_imports0: Test(3) {
    my $r = Imports0->from_file('src/fixed_struct.bin');

    is($r->two(), 0x50, 'Equals');
    is($r->hw()->one(), 0x41, 'Equals');
    is($r->hw_one(), 0x41, 'Equals');
}

Test::Class->runtests;
