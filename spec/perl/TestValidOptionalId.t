# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestValidOptionalId;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ValidOptionalId;

sub test_valid_optional_id: Test(0) {
    my $r = ValidOptionalId->from_file('src/fixed_struct.bin');

}

Test::Class->runtests;
