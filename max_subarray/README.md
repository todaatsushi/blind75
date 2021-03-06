# 53. Maximum Subarray
[Link](https://leetcode.com/problems/maximum-subarray/)

```
Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

A subarray is a contiguous part of an array.

Constraints:
    1 <= nums.length <= 105
    -104 <= nums[i] <= 104

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
Compute every contiguous array within `nums` and get the subtotal of each of those subarrays.

Time: O(N^^3)
Space: O(1)

We would loop through twice (once for the start, once to build the subarray) and finally, another O(N)

## Possible solutions
- Iterate inwards using 2 pointers at each end

Any sub array is going to, at max, include all elements. Given that the total will only get:
    - Smaller as we move a positive number out of the scope
    - Larger as move a negative number out of the scope

We will still have to be careful of the false positive (ie. the larger int followed by a huge negative)

- Iterate outwards

Same problem as above.


## Solution
Use a variation of a sliding window, more like a runner and chaser instead.

Using those as the two indicies of the subarray, we calculate the total. When adding a new number to the subarray, if
the total of the prefix is negative, we can discount it as it doesn't contribute to making a new total larger. In that
case, we can just drop the prefix and move the chaser to the location of the runner.


Time: O(N)
Space: O(1)

We only go over the elements of `nums` once, and given we only store the subtotals at any given time, no extra memory is
needed.
