const productExceptSelf = (nums: number[]): number[] => {
  let productNumbers = []
  let cumulativeProduct = 1

  for (let index = 0; index < nums.length; index++) {
    productNumbers.push(cumulativeProduct)
    cumulativeProduct *= nums[index]
  }

  cumulativeProduct = 1
  for (let index = nums.length - 1; index > -1; index--) {
    productNumbers[index] *= cumulativeProduct
    cumulativeProduct *= nums[index]
  }
  return productNumbers
}

export default productExceptSelf
