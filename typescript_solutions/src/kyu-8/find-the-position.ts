export function position(letter: string): string {
    const p = letter.charCodeAt(0) - "a".charCodeAt(0) + 1;
    return `Position of alphabet: ${p}`
}


import { assert } from "chai";

describe("solution", function () {
    it("Basic tests", function () {
        assert.strictEqual(position("a"), "Position of alphabet: 1");
        assert.strictEqual(position("z"), "Position of alphabet: 26");
        assert.strictEqual(position("e"), "Position of alphabet: 5");
    });
});