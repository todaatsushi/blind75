# 20. Valid Parentheses
[Link](https://leetcode.com/problems/valid-parentheses/)

```
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

Open brackets must be closed by the same type of brackets.
Open brackets must be closed in the correct order.

Constraints:
    - 1 <= s.length <= 104
    - s consists of parentheses only '()[]{}'.
```

# Breakdown
Simple example of LIFO in action where the complement of the items into the data structure has to be the first out ie.
the closing brackets in this case.

# Examples
Input: s = "()"
Output: true

Input: s = "()[]{}"
Output: true

Input: s = "(]"
Output: false

## Brute force

Keep a data structure listing the order of opening or closing. Iterate on every closing through a list of brackets and
check with the notary structure.


Time: O(N)
Space: O(N)

Each opening adds to a structure to be traversed and every closing is mapped to all the closing brackets that are
constant time.
Size of data structure increases as the input increases.

## Possible solutions
- Same as the brute force but we improve the efficiency of each operation ie. use a hash table to lookup and a stack as
  a notary.
    - No change to Big O

## Solution
Init stack
Create bracket mapper and populate with K: V where K is closing, V is opening
loop over `s`
if `s` is in the mapper ie. is closing, peek the stack and compare.
    - if stack is empty, return `False`
    - if the stack doesn't match the lookup
        - return `False`
    - else pop
else add the character to the stack

Time: O(N)
Space: O(N)

Both operations and space grow with input `s`
