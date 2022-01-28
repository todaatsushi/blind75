# 238. Product of Array Except Self
[Link](https://leetcode.com/problems/product-of-array-except-self/)

```
Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

You must write an algorithm that runs in O(n) time and without using the division operation.

Constraints:
    - 2 <= nums.length <= 105
    - 30 <= nums[i] <= 30

The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
```

# Breakdown
Without using divide, and in O(N) time, we have to recalculate the array where every element is a pivot that represents
the product of the rest of the array.

# Examples
Input: nums = [1,2,3,4]
Output: [24,12,8,6]

Input: nums = [-1,1,0,-3,3]
Output: [0,0,9,0,0]

## Brute force

While not O(N), we could calculate the product with the other elements by looping through the array, and get the two sub
arrays from the element.

The sub array would then be looped over, keeping a cumulative product result which is then inserted in the result index
at the elements' indicies.

Time: O(N^^2)
Space: O(N)

The double for loop creates a quadratic growth where none of the elements initialised are affected by that nested loop,
only the storage lists ie O(N).

## Possible solutions

## Solution


Time: O()
Space: O()
