package spec::perl::TestStrEncodingsEscapingToS;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use StrEncodingsEscapingToS;
use Test::Exception;

sub test_str_encodings_escaping_to_s: Test(4) {
    my $r = StrEncodingsEscapingToS->from_file('src/str_encodings.bin');

    throws_ok { $r->str1() } qr/^Unknown encoding 'ASCII\\\\x'/;
    throws_ok { $r->str2() } qr/^Unknown encoding 'UTF-8\\'x'/;
    throws_ok { $r->str3() } qr/^Unknown encoding 'SJIS\\\"x'/;
    throws_ok { $r->str4() } qr/^Unknown encoding 'IBM437\\nx'/;
}

Test::Class->runtests;
