function isPangram(string) {
    let alphabet = ''
    const processed = string.toLowerCase()
    for (let charCode = 'a'.charCodeAt(); charCode <= 'z'.charCodeAt(); charCode++) {
        alphabet += String.fromCharCode(charCode)
    }
    for (letter of alphabet) {
        if (!processed.match(letter)) return false
    }
    return true
}

const chai = require("chai");
const assert = chai.assert;
chai.config.truncateThreshold = 0;

describe("Tests", () => {
    it("test1", () => {
        var string = "The quick brown fox jumps over the lazy dog."
        assert.strictEqual(isPangram(string), true)
    })
    it("test2", () => {
        var string = "This is not a pangram."
        assert.strictEqual(isPangram(string), false)
    })
    it("test3", () => {
        var string = "Cwm fjord bank glyphs vext quiz"
        assert.strictEqual(isPangram(string), true)
    })

});
