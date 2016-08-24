#!/usr/bin/env perl

use strict;
use warnings;
use TAP::Harness::JUnit;

my $harness = TAP::Harness::JUnit->new( {
    xmlfile => 'test_out/perl/output.xml',
} );

my @tests = (
    'spec/perl/TestHelloWorld.t',
    'spec/perl/TestProcessXorValue.t',
    'spec/perl/TestRepeatEosStruct.t',
    'spec/perl/TestTermStrz.t',
);

$harness->runtests(@tests);
