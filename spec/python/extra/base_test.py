import unittest

class BaseTest(unittest.TestCase):
    """
    Base class for some KaitaiStruct tests that uses RegEx assertions.

    - Python 2 has only `assertRaisesRegexp` method
    - Python 3.11 and below have both `assertRaisesRegex` and `assertRaisesRegexp`
    - Python 2.12 and above has only `assertRaisesRegex`

    The tests run on both Python 2 & 3, so this base class adds missing method for
    compatibility, therefore tests written for latest Python will work on older versions.
    """
    def __init__(self, *args, **kwargs):
        super(BaseTest, self).__init__(*args, **kwargs)
        if not hasattr(unittest.TestCase, 'assertRaisesRegex'):
            setattr(unittest.TestCase, 'assertRaisesRegex', unittest.TestCase.assertRaisesRegexp)