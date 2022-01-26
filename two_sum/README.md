# 1. Two Sum
[Link](https://leetcode.com/problems/two-sum/)

```
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

Constraints:
    2 <= nums.length <= 104
    -109 <= nums[i] <= 109
    -109 <= target <= 109
    Only one valid answer exists.
```

# Breakdown

## Brute force
- Given that a solution is guaranteed to exist, we could brute force the problem by iterating over every element, and
  matching comparing the element to every other item in the list, returning both indicies when we find the answer.

  O(N^^2) Time
  O(1) Space

Two loops over `n` elements that will grow as `n` does. The list of two indicies will stay the same regardless of the
size of `n`.

## Solution
- There is only one answer ie. one complement to the answer element in `n`. 
- We can hold all the values we have been to already with the index and compare every element to the values in the lookup.
- We can store the complement to the visted values as a key with the value as the index.
- If an element's complement appears in the lookup, those two indicies are the answer.

O(N) space + time

One loop over `n` and the size of the lookup struct grows as `n` does.
