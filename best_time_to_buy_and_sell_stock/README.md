# 121. Best Time to Buy and Sell Stock
[Link](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/)

```
You are given an array prices where prices[i] is the price of a given stock on the ith day.

You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

Constraints:
    1 <= prices.length <= 105
    0 <= prices[i] <= 104
```

# Breakdown
If x is the index of day 1 (of two, not index 0) and y is the index of day 2, any profit will be `prices[x] -
prices[y]` where the profit is over 0.

As we loop through the list, we only need to set `x` where `y` is bigger than `x`. That is, if we are making a loss,
`x` becomes `y` and `y` becomes `y + 1`. At every instance where `y` is increasing, we just need to compare the max
profit to current profit and reset.

To avoid attributing to a loss, we can set the default loss at `0`.

## Brute force
To get the max profit, the BF method would be to calculate all possible permutations of every day, with every subsequent
day, before returning the largest number above 0.

Time: O(N)
Space: O(N)

Both the profit stores and the operation counts of O(1) scale with the size of the input.

# Examples
Input: prices = [7,1,5,3,6,4]
Output: 5

Input: prices = [7,6,4,3,1]
Output: 0

Input: prices = [3,10,2,1,8,20,4]
Output: 19

## Possible solutions
- The max is either going to be the difference between the lowest and the highest after that lowest OR the highest and
  the difference between the highest and the lowest before the highest.
    - Loop through the list 2 times, fetching the highest and lowest.
    - Calculate the profits vs 0 on both and compare.

- Two pointers, buy and sell starting from each end of the prices list.
- Calculate the profit and compare to current max:
    - If profit is negative, move buy inwards.
    - If profit is positive, move sell inwards.
        - Compare to current max.
- <em>Fails: in a scenario where a "false" positive prevents the buy from progressing further.</em>

- Two pointers, buy and sell starting from 0, 1.
- Enter a while loop while profit is increasing, we move sell.
- When profit is negative, we move buy.
- Exit at the end of prices.

## Solution
Choose #3:

init 0 profit
if prices.length is 1, return profit
init 2 pointers, 0 + 1
while sell is in range
    get buy and sell vals + calc current profit
    if current profit 
        compare and set if necessary
    else
        buy becomes sell
    move sell


