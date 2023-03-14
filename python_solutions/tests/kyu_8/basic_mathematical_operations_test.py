import codewars_test as test
from python_solutions.kyu_8.basic_mathematical_operations import basic_op

@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(basic_op('+', 4, 7), 11)
        test.assert_equals(basic_op('-', 15, 18), -3)
        test.assert_equals(basic_op('*', 5, 5), 25)
        test.assert_equals(basic_op('/', 49, 7), 7)