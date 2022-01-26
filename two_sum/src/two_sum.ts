const twoSum = (nums: number[], target: number): (number[] | undefined) =>{
  let lookUp: {[complement: string]: number} = {}
  let complement: string;

  for (const [index, element] of nums.entries()) {
    complement = `${target - element}`

    if (complement in lookUp) {
      return [index, lookUp[complement]]
    }
    lookUp[element] = index
  }
}

export default twoSum
