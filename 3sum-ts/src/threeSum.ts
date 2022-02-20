const checkArrayHasAllElements = (checkingFor: Array<any>, checkingIn: Array<any>): boolean => {
  return checkingIn.every((value: any) => checkingFor.includes(value))
}

const threeSum = (nums: number[]): Array<Array<number>> => {
  nums = nums.sort((a: number, b: number): number => a - b)
  let combinations: Array<Array<number>> = []

  let isUnique: boolean
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
      if (subTotal === num * -1) {
        isUnique = true
        for (let solution of combinations) {
          if (checkArrayHasAllElements([nums[left], nums[right], num], solution)) {
            isUnique = false
          }
        }
        if (isUnique) {
          combinations.push([nums[left], nums[right], num])
        }
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
 
export default threeSum
