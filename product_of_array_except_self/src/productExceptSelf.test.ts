import productExceptSelf from "./productExceptSelf"

test("Computes expected values correctly", () => {
  const nums: number[] = [1,2,3,4]
  const expected: number[] = [24,12,8,6]
  expect(productExceptSelf(nums)).toEqual(expect.arrayContaining(expected))
})
