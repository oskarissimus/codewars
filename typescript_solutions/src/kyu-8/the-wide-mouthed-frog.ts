export function mouthSize(animal: string): string {
    return animal.toLowerCase() == "alligator" ? "small" : "wide"
}

import { assert } from "chai";

describe("Basic tests", () => {
    it("Basic tests should pass", () => {
        assert.equal(mouthSize("toucan"), "wide");
        assert.equal(mouthSize("ant bear"), "wide");
        assert.equal(mouthSize("alligator"), "small");
    });
});