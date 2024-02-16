#!/usr/bin/env python

import sys
import unittest
import xmlrunner

if __name__ == '__main__':
    with open(sys.argv[1], 'wb') as output:
        # We could run `python -m xmlrunner` directly (thus avoiding this
        # `run-python-xml.py` script altogether), but a wrapper script like this
        # allows us to adjust `xmlrunner` settings that cannot be changed from
        # the command line, such as `outsuffix` in unittest-xml-reporting 2.5.2
        # (which is the last version working in Python 2).
        unittest.main(
            module=None,
            argv=[sys.argv[0] + ' OUTFILE'] + sys.argv[2:],
            testRunner=xmlrunner.XMLTestRunner(output=output, outsuffix=''),
            # these make sure that some options that are not applicable
            # remain hidden from the help menu.
            failfast=False, buffer=False, catchbreak=False)
