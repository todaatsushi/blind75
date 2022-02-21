import { threeSum, checkArrayHasAllElements } from "./threeSum"

test("Test nums creates an array with the correct results", () => {
  const nums: Array<number> = [-1,0,1,2,-1,-4]
  const expected: Array<Array<number>> = [[-1, 2, -1], [-1, 0 ,1]]

  let passed: boolean = false
  const output: Array<Array<number>> = threeSum(nums)

  for (let solution of output) {
    if (checkArrayHasAllElements(solution, expected)) {
      passed = true
    }
  }

  if (output.length !== expected.length) {
    passed = false
  }

  expect(passed).toEqual(true)
})

test("Empty input returns no results", () => {
  const nums: Array<number> = []
  const expected: Array<number> = []

  expect(threeSum(nums)).toEqual(expect.arrayContaining(expected))
})

test("No answer input returns no results", () => {
  const nums: Array<number> = [0]
  const expected: Array<number> = []

  expect(threeSum(nums)).toEqual(expect.arrayContaining(expected))
})
