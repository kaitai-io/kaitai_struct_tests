#!/usr/bin/env python

import sys
import unittest2
import xmlrunner

loader = unittest2.TestLoader()
tests = loader.discover(sys.argv[1])
testRunner = xmlrunner.XMLTestRunner(output = sys.argv[2])
testRunner.run(tests)
