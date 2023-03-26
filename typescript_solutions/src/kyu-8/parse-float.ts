export function parseF(s: string): number | null {
    const parsed = Number(s)
    return isNaN(parsed) ? null : parsed
}

import { assert } from "chai";

describe("Parse float", function () {
    it("Fixed tests", function () {
        assert.equal(parseF("1"), 1.0);
        assert.equal(parseF("true"), null);
    });
});