import unittest

from src.app import greet


class AppTests(unittest.TestCase):
    def test_greet(self) -> None:
        self.assertEqual(greet("world"), "hello, world!")


if __name__ == "__main__":
    unittest.main()
