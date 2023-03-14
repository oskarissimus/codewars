def basic_op(operator, value1, value2):
    operator_to_function = {
        "*": lambda a,b: a*b,
        "/": lambda a,b: a/b,
        "+": lambda a,b: a+b,
        "-": lambda a,b: a-b,
    }
    return operator_to_function[operator](value1,value2)