import maxProfit from "./maxProfit"

test("test we get the best price when a profit is to be made.", () => {
  const prices: number[] = [7,1,5,3,6,4]
  const profit: number = 5

  expect(maxProfit(prices)).toBe(profit)
})

test("0 profit when prices are declining", () => {
  const prices: number[] = [7,6,4,3,1]
  const profit: number = 0

  expect(maxProfit(prices)).toBe(profit)
})

test("get best price with non optimal days", () => {
  const prices: number[] = [3,10,2,1,8,20,4]
  const profit: number = 19

  expect(maxProfit(prices)).toBe(profit)
})

test("get best price with duplicated days", () => {
  const prices: number[] = [3,20,2,1,8,20,1]
  const profit: number = 19

  expect(maxProfit(prices)).toBe(profit)
})

test("return 0 for only 1 price", () => {
  const prices: number[] = [3]
  const profit: number = 0

  expect(maxProfit(prices)).toBe(profit)
})
