export function howManyLightsabersDoYouOwn(name?: any): number {
    if (name == "Zach") return 18
    return 0
}

// The question mark (?) after name attribute marks it as optional.
import { assert } from "chai";

describe("How many light sabers?", function () {
    it("Basic tests", function () {
        assert.equal(howManyLightsabersDoYouOwn('Zach'), 18);
        assert.equal(howManyLightsabersDoYouOwn('Adam'), 0);
    });
});