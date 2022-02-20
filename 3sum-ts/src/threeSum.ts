const threeSum = (nums: number[]): Array<Array<number>> => {
  nums = nums.sort((a: number, b: number): number => a - b)
  let combinations: Array<Array<number>> = []

  let left: number
  let right: number
  let subTotal: number
  
  nums.forEach((num: number, index: number): void => {
    left = 0
    right = nums.length - 1

    while (left < right) {
      // Don't count itself in the calculation
      switch (index) {
        case right:
          right--
          break
        case left:
          left++
          break
      }

      subTotal = nums[left] + nums[right]
      if (subTotal === num) {
        // TODO - only add to combos if unique
        combinations.push([nums[left], nums[right], num])
        left++
      } else if (subTotal < num) {
        left++
      } else {
        right--
      }
    }
  })
  
  return combinations
}
 
console.log(threeSum([-1, 0, 1, 2, -1, -4]))
