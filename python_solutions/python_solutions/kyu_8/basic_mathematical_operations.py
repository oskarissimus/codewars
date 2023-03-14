def basic_op(operator, value1, value2):
    operator_to_function = {
        "*": lambda a,b: a*b,
        "/": lambda a,b: a/b,
        "+": lambda a,b: a+b,
        "-": lambda a,b: a-b,
    }
    return operator_to_function[operator](value1,value2)

import codewars_test as test

@test.describe("Fixed Tests")
def fixed_tests():
    @test.it('Basic Test Cases')
    def basic_test_cases():
        test.assert_equals(basic_op('+', 4, 7), 11)
        test.assert_equals(basic_op('-', 15, 18), -3)
        test.assert_equals(basic_op('*', 5, 5), 25)
        test.assert_equals(basic_op('/', 49, 7), 7)