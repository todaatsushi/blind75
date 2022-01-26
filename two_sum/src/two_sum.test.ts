import twoSum from "./two_sum"

test("finds correct indicies in array", () => {
  const nums: number[] = [2,7,11,15]
  const target: number = 9
  expect(twoSum(
    nums, target
  )).toEqual(expect.arrayContaining([0, 1]))
})
