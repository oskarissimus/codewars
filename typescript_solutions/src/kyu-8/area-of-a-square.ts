export const squareArea = (num: number): number => {
    return (2 * num / Math.PI) ** 2
}

import { assert } from "chai";


const DELTA = 0.05

describe("Fixed tests", function () {
    it('Testing for 2', function () {
        assert.approximately(squareArea(2), 1.62, DELTA);
    });
    it('Testing for 0', function () {
        assert.approximately(squareArea(0), 0, DELTA);
    });
    it('Testing for 14.05', function () {
        assert.approximately(squareArea(14.05), 80, DELTA);
    });
});