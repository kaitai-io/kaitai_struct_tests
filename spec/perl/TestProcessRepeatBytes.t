# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestProcessRepeatBytes;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ProcessRepeatBytes;

sub test_process_repeat_bytes: Test(2) {
    my $r = ProcessRepeatBytes->from_file('src/process_xor_4.bin');

    is(@{$r->bufs()}[0], pack('C*', (114, 37, 61, 138, 20)), 'Equals');
    is(@{$r->bufs()}[1], pack('C*', (74, 82, 170, 16, 68)), 'Equals');
}

Test::Class->runtests;
