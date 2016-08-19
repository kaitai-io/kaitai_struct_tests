package spec::perl::TestHelloWorld;

use base qw(Test::Class);
use Test::More;
use HelloWorld;

sub test_hello_world : Test {
    my $r = HelloWorld->from_file('src/fixed_struct.bin');
    is($r->{one}, 0x50, 'equals');
}

Test::Class->runtests;
