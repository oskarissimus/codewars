function basicOp(operation, value1, value2) {
    const operation_to_function = {
        "*": (a, b) => a * b,
        "/": (a, b) => a / b,
        "+": (a, b) => a + b,
        "-": (a, b) => a - b,
    }
    return operation_to_function[operation](value1, value2)
}

const Test = require('@codewars/test-compat');

describe("Tests", () => {
    it("test", () => {
        console.log("Basic tests\n");
        Test.assertSimilar(basicOp('+', 4, 7), 11);
        Test.assertSimilar(basicOp('-', 15, 18), -3);
        Test.assertSimilar(basicOp('*', 5, 5), 25);
        Test.assertSimilar(basicOp('/', 49, 7), 7);
    });
});