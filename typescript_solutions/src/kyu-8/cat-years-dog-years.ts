export function humanYearsCatYearsDogYears(humanYears: number): [number, number, number] {
  let catYears
  if (humanYears == 1) catYears = 15
  else if (humanYears == 2) catYears = 24
  else catYears = 24 + 4 * (humanYears - 2)

  let dogYears
  if (humanYears == 1) dogYears = 15
  else if (humanYears == 2) dogYears = 24
  else dogYears = 24 + 5 * (humanYears - 2)

  return [humanYears, catYears, dogYears]
}
// See https://www.chaijs.com for how to use Chai.
import { assert } from "chai";

describe("Example Tests", function () {

  it("one", function () {
    assert.deepEqual(humanYearsCatYearsDogYears(1), [1, 15, 15]);
  });

  it("two", function () {
    assert.deepEqual(humanYearsCatYearsDogYears(2), [2, 24, 24]);
  });

  it("ten", function () {
    assert.deepEqual(humanYearsCatYearsDogYears(10), [10, 56, 64]);
  });

});