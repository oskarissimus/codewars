export function binaryArrayToNumber(arr: number[]): number {
    return arr[0] * 8 + arr[1] * 4 + arr[2] * 2 + arr[3]
};

import { assert } from "chai";

describe("One's and Zero's", () => {
    it("Example tests", () => {
        assert.strictEqual(binaryArrayToNumber([0, 0, 0, 1]), 1);
        assert.strictEqual(binaryArrayToNumber([0, 0, 1, 0]), 2);
        assert.strictEqual(binaryArrayToNumber([1, 1, 1, 1]), 15);
        assert.strictEqual(binaryArrayToNumber([0, 1, 1, 0]), 6);
    });
});
