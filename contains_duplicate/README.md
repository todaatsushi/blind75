# 217. Contains Duplicate
[Link](https://leetcode.com/problems/contains-duplicate/)

```
Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

Constraints:
    1 <= nums.length <= 105
    -109 <= nums[i] <= 109
```

# Breakdown

## Brute force
There are multiple built in ways to solve this problem, the BF would be to loop through `nums`, and comparing the
element to the subarray of the rest of `nums`.

O(N^^2) time
O(N^^2) space

Both the storage and number of operations will grow as `nums` does, quadratic as we compare every other element to the
current element.

## Possible solutions
- Cast `nums` into a `set` and compare the lengths 
    - Although the shortest code wise, and not affecting O(), this method requires 2 passes of `nums` and 1 pass of the
      set from len calls.
- Loop over `nums` and add to a hashmap, return `false` if the item already exists
    - An overengineered solution as it doesn't fulfil the core functionality of the data structure (key: value).

## Solution
- Initialise an empty hashmap.
- Loop over all the elements in `nums`
- Check the hashmap for the current element
    - if the key exists, return `true`
    - else `false`

O(N) in space and time
Operations scale with size of the input, and so does the size of the hashmap.
