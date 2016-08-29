package spec::perl::TestStrEos;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use StrEos;

sub test_str_eos: Test {
    my $r = StrEos->from_file("src/term_strz.bin");

    is($r->{str}, "foo|bar|baz@"}, 'Equals');
}

Test::Class->runtests;
