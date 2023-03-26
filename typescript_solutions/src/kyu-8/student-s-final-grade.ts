export function finalGrade(exam: number, projects: number): number {
    if (exam > 90 || projects > 10) return 100
    if (exam > 70 && projects >= 5) return 90
    if (exam > 50 && projects >= 2) return 75
    return 0
}

import { assert } from "chai";


describe("Student's Final Grade", () => {
    it("Fixed tests", () => {
        assert.strictEqual(finalGrade(100, 12), 100);
        assert.strictEqual(finalGrade(85, 5), 90);
        assert.strictEqual(finalGrade(71, 5), 75);
    });
});
