export const isPangram = (phrase: string): boolean => {
    const matches = [...phrase.toLowerCase().matchAll(/[a-z]/g)];
    const set = new Set(matches.map((match: RegExpMatchArray) => match[0]));
    return set.size === 26;
};


// See https://www.chaijs.com for how to use Chai.
import { assert } from "chai";

describe("example", function () {
    it("test", function () {
        assert.strictEqual(isPangram("The quick brown fox jumps over the lazy dog."), true);
        assert.strictEqual(isPangram("This is not a pangram."), false);
    });
});