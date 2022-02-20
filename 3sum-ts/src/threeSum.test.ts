import threeSum from "./threeSum"

test("Test nums creates an array with the correct results", () => {
  const nums: Array<number> = [-1,0,1,2,-1,-4]
  const expected: Array<Array<number>> = [[-1, 2, -1], [-1, 0 ,1]]

  expect(threeSum(nums)).toBe(expected)
})

test("Empty input returns no results", () => {
  const nums: Array<number> = []
  const expected: Array<number> = []

  expect(threeSum(nums)).toBe(expected)
})

test("No answer input returns no results", () => {
  const nums: Array<number> = [0]
  const expected: Array<number> = []

  expect(threeSum(nums)).toBe(expected)
})
