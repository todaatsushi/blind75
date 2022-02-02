# 53. Maximum Subarray
[Link](https://leetcode.com/problems/maximum-subarray/)

```
Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

A subarray is a contiguous part of an array.

Constraints:
```

# Breakdown
The main problem is to figure out how we would make sure that we don't miss any potential higher summing arrays by
adding a catch all rule, such as changing the boundries if a new array makes a subtotal smaller.

That being said, the approach would be to ensure that the Big O of the solution is within linear complexity.

Most problems that require a subset of any iterable tend to be well approached using a sliding window approach. However,
as in the case of example #1 below, we have to be careful not to discount -1 in order to count the 2.

# Examples
Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
Output: 6

Input: nums = [1]
Output: 1

Input: nums = [5,4,-1,7,8]
Output: 23

## Brute force
Compute every contiguous array within `nums` and get the greatest sum.

Time: O(N^^2)
Space: O(N^^2)

We would be performing N operations N times where N is the length of `nums`. The space used to store the subarrays would
grow with the number of subarrays and, therefore, will also grow at the same rate.

## Possible solutions

## Solution


Time: O()
Space: O()
