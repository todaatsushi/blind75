import unittest

from mapper import PARENTHESIS_MAPPER
from valid_parentheses import valid_parentheses


class ValidParenthesisTestCase(unittest.TestCase):
    def test_simple_parenthesis(self):
        self.assertTrue(
            valid_parentheses("()"
        ))

    def test_combo_parenthesis(self):
        self.assertTrue(valid_parentheses("()[]{}"))

    def test_fail(self):
        self.assertFalse(valid_parentheses("(}"))

if __name__ == "__main__":
    unittest.main()
