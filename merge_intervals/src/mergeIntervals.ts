const intervalsOverlapping = (leftInterval: number[], rightInterval: number[]): boolean => {
  return [
    leftInterval[1] >= rightInterval[0],
    rightInterval[0] <= leftInterval[1]
  ].some((element: boolean) => element === true)
}

const merge = (leftInterval: number[], rightInterval: number[]): number[] => {
  return [
    Math.min(leftInterval[0], rightInterval[0]),
    Math.max(leftInterval[1], rightInterval[1])
  ]
}

const mergeIntervals = (intervals: number[][]): number[][] => {
  let left: number = 0
  let right: number = 1
  let rightInterval: number[]
  let leftInterval: number[]

  while (right < intervals.length) {
    rightInterval = intervals[right]
    leftInterval = intervals[left]

    if (intervalsOverlapping(leftInterval, rightInterval)) {
      intervals[left] = merge(leftInterval, rightInterval)
      intervals.splice(right, 1)
    } else {
      left++
      right++
    }
  }

  return intervals
}

export default mergeIntervals
