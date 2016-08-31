#!/usr/bin/env perl

use strict;
use warnings;
use TAP::Harness::JUnit;

my $harness = TAP::Harness::JUnit->new( {
    xmlfile => 'test_out/perl/output.xml',
} );

my @tests = (
    'spec/perl/TestBufferedStruct.t',
    'spec/perl/TestDefaultBigEndian.t',
    'spec/perl/TestEnum0.t',
#    'spec/perl/TestEnumIf.t',
#    'spec/perl/TestEofExceptionBytes.t',
#    'spec/perl/TestExpr0.t',
#    'spec/perl/TestExpr1.t',
#    'spec/perl/TestExpr2.t',
#    'spec/perl/TestFixedStruct.t',
    'spec/perl/TestFloatingPoints.t',
    'spec/perl/TestHelloWorld.t',
#    'spec/perl/TestIfStruct.t',
#    'spec/perl/TestInstanceIoUser.t',
#    'spec/perl/TestInstanceStd.t'
#    'spec/perl/TestInstanceStdArray.t'
#    'spec/perl/TestInstanceUserArray.t',
#    'spec/perl/TestMultipleUse.t',
#    'spec/perl/TestNavParent.t',
#    'spec/perl/TestNavRoot.t',
#    'spec/perl/TestPositionAbs.t',
#    'spec/perl/TestPositionInSeq.t',
#    'spec/perl/TestPositionToEnd.t',
    'spec/perl/TestProcessRotate.t',
#    'spec/perl/TestProcessToUser.t',
#    'spec/perl/TestProcessXor4Const.t',
#    'spec/perl/TestProcessXor4Value.t',
#    'spec/perl/TestProcessXorConst.t',
#    'spec/perl/TestProcessXorValue.t',
    'spec/perl/TestRepeatEosStruct.t',
#    'spec/perl/TestRepeatEosU4.t',
#    'spec/perl/TestRepeatNStruct.t',
#    'spec/perl/TestRepeatNStrz.t',
#    'spec/perl/TestRepeatUntilComplex.t',
#    'spec/perl/TestRepeatUntilS4.t',
#    'spec/perl/TestStrEncodings.t',
#    'spec/perl/TestStrEos.t',
    'spec/perl/TestTermStrz.t',
#    'spec/perl/TestTypeIntUnaryOp.t',
#    'spec/perl/TestZlibWithHeader78.t',
);

$harness->runtests(@tests);
