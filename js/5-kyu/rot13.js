function rot13(message) {
    return message.split("").map(letter => rotateLetter(letter)).join("")
}

function rotateLetter(letter) {
    if (!letter.match(/[a-zA-Z]/)) return letter
    const isUpperCase = letter.toUpperCase() == letter
    const first = (isUpperCase ? 'A' : 'a').charCodeAt()
    const charCode = letter.charCodeAt()
    const rotatedCharCode = ((charCode - first) + 13) % 26 + first
    return String.fromCharCode(rotatedCharCode)
}

const chai = require("chai");
const assert = chai.assert;

describe("Tests", function () {
    it("Sample tests", function () {
        for (const [input, expected] of [
            ["test", "grfg"],
            ["Test", "Grfg"],
            ['Ruby is cool!', 'Ehol vf pbby!']
        ]) {
            assert.strictEqual(rot13(input), expected, `Test failed with messsage = '${input}'`);
        }
    });
});
