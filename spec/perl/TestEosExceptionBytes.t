package spec::perl::TestEosExceptionBytes;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use EosExceptionBytes;
use Test::Exception;

sub test_eos_exception_bytes: Test(1) {
    dies_ok { EosExceptionBytes->from_file('src/term_strz.bin') } 'Died';
}

Test::Class->runtests;
