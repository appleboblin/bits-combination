import unittest
import timeit
from bits import generate_bits

class TestBits(unittest.TestCase):

    def test_bits(self):
        # test cases
        test_case = [
            (0, ['']),
            (1, ['0', '1']),
            (2, ['00', '10', '01', '11']),
            (3, ['000', '100', '010', '110', '001', '101', '011', '111']),
        ]

        # test case
        for bit_length, expected_combination in test_case:
            with self.subTest(bit_length=bit_length):
                combinations = generate_bits(bit_length)
                # test number of combinations
                self.assertEqual(len(combinations), len(expected_combination))

                # test actual combinations
                for bits in combinations:
                    self.assertEqual(len(bits), bit_length)
                self.assertEqual(combinations, expected_combination)

    def test_timing(self):
        bit_length = 30
        # measure time
        time = timeit.timeit(lambda: generate_bits(bit_length), number=1)
        print(f"Execution time for {bit_length}-bits: {time:.6f} seconds")

        # # test if correct length
        # combinations = generate_bits(bit_length)
        # self.assertEqual(len(combinations), 2**bit_length)

if __name__ == "__main__":
    unittest.main()