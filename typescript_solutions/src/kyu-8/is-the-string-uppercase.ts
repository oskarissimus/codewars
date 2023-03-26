export function isUpperCase(str: string): boolean {
    return !str.match(/[a-z]/)
}

import { assert } from "chai";

describe('Fixed Tests', () => {
    it("Testing for c", () => assert.strictEqual(isUpperCase('c'), false));
    it("Testing for C", () => assert.strictEqual(isUpperCase('C'), true));
    it("Testing for hello I AM DONALD", () => assert.strictEqual(isUpperCase('hello I AM DONALD'), false));
    it("Testing for HELLO I AM DONALD", () => assert.strictEqual(isUpperCase('HELLO I AM DONALD'), true));
    it("Testing for ACSKLDFJSgSKLDFJSKLDFJ", () => assert.strictEqual(isUpperCase('ACSKLDFJSgSKLDFJSKLDFJ'), false));
    it("Testing for ACSKLDFJSGSKLDFJSKLDFJ", () => assert.strictEqual(isUpperCase('ACSKLDFJSGSKLDFJSKLDFJ'), true));
});