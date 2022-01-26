const maxProfit = (prices: number[]): number => {
  let profit: number = 0
  if (prices.length === 1) {
      return profit
  }

  let buy: number = 0
  let sell: number = 1

  let sellPrice: number
  let buyPrice: number
  let currentProfit: number

  do {
    buyPrice = prices[buy]
    sellPrice = prices[sell]
    currentProfit = sellPrice - buyPrice

    if (currentProfit > 0) {
      profit = Math.max(profit, currentProfit)
    } else {
      buy = sell
    }
    sell++
  } while (sell < prices.length)

  return profit
}

export default maxProfit
