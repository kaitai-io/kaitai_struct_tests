# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestValidEqStrEncodings;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ValidEqStrEncodings;

sub test_valid_eq_str_encodings: Test(0) {
    my $r = ValidEqStrEncodings->from_file('src/str_encodings.bin');

}

Test::Class->runtests;
