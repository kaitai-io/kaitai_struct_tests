package spec::perl::TestEofExceptionBytes;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use EofExceptionBytes;
use Test::Exception;

sub test_eof_exception_bytes: Test(1) {
    dies_ok { EofExceptionBytes->from_file('src/term_strz.bin') } 'Died';
}

Test::Class->runtests;
