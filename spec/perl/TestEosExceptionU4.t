package spec::perl::TestEosExceptionU4;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use EosExceptionU4;
use Test::Exception;

sub test_eos_exception_u4: Test(1) {
    dies_ok { EosExceptionU4->from_file('src/term_strz.bin') } 'Died';
}

Test::Class->runtests;
