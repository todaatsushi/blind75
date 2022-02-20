# 15. 3Sum
[Link](https://leetcode.com/problems/3sum/)

```
Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

Notice that the solution set must not contain duplicate triplets.

Constraints:
    0 <= nums.length <= 3000
    -105 <= nums[i] <= 105
```

# Breakdown
An extension to the first LC question where we return the indicies to the target number given that a solution exists.

In this case, we have 3 extra considerations:
    - There is no guaranteed solution
    - Answers must be unique (ie. no dupe triplets)
    - There are three numbers which must add up to 0

# Examples
input: nums = [-1,0,1,2,-1,-4]
output: [[-1,-1,2],[-1,0,1]]

Input: nums = []
Output: []

Input: nums = [0]
Output: []

## Brute force
Come up with every triplet possible and find the sum answer to it.

Time: O(N^^3)
Space: O(N)

For every number, we'll need to find another, and then yet another. Each of those layers requires a loop which makes
this cubic.

For space, the data that grows the most is the result array, the most of would be every element of the array, therefore
making it grow with the size of the input.

## Possible solutions
1) Loop through `nums` and calculate 2sum on the negative version of that number. We store the target and successful combos
in a map in order to not repeat the solutions. In the case the 2sum is successful and not existing in the map, we add to
the map and add the Vec to the result.

2Sum works by finding the complement of the target less either of the two inputs within an arr, and, in this case,
returns both values if those exist.

Time: O(N^^2)
Space: O(N)

Will need a linear 2sum for every num in `nums` and store those in both a result Vec and also a hashmap, which will have
all the nums in `nums` at most.

2) Loop through `nums` after sorting it and initialise two pointers on each loop. We can move left, right inwards depending
on whether the total is too large or too small, ignoring the `num` when either lands on it.

Time: O(N^^2)
Space: O(N)

The result list will grow with the input in the worst case.
