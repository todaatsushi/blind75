# 56. Merge Intervals
[Link](https://leetcode.com/problems/merge-intervals/)

```
Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.

Constraints:
- 1 <= intervals.length <= 104
- intervals[i].length == 2
- 0 <= starti <= endi <= 104
```

# Breakdown

Given a list of lists where the sublists are the lower and upper bounds of an interval, we have to come up with an
algorithm to merge any overlapping intervals and therefore condense the list.

The complexity comes from multiple intervals being covered by multiple sublists and start/ends not necessarily being
organised in a nice manner.

# Examples

Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
Output: [[1,6],[8,10],[15,18]]

Input: intervals = [[1,4],[4,5]]
Output: [[1,5]]

## Brute force

Loop over `intervals` and see which intervals it can be merged into.   

Time: O(N^^2)
Space: O(1)

Each interval needs to be looped over the rest of the list.

## Solution
1) Two pointers

Iterate the entire range of `intervals` - right and then left. Compare each interval to the next and check if they
should merge.

If yes, take lower of both lower bounds and the higher of both upper bounds.

Should merge should check that the upper of left is more than or equal to upper of right

Carry on while right index is lower than the length of intervals.

Time: O(N)
Space: O(1)

Loop over the array once and no extra growing vars.
