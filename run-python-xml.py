#!/usr/bin/env python

import sys
import unittest
import xmlrunner

loader = unittest.TestLoader()
tests = loader.discover(sys.argv[1])
testRunner = xmlrunner.XMLTestRunner(output = sys.argv[2], outsuffix='')
testRunner.run(tests)
