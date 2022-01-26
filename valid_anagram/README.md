# 242. Valid Anagram
[Link](https://leetcode.com/problems/valid-anagram/)

```
Given two strings s and t, return true if t is an anagram of s, and false otherwise.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

Constraints:
    1 <= s.length, t.length <= 5 * 104
    s and t consist of lowercase English letters.
```

# Breakdown
If a word is an anagram, both words will have the same number of letters and both sets of letters will appear the same
amount of times.

## Examples
```
Input: s = "anagram", t = "nagaram"
Output: true

Input: s = "rat", t = "car"
Output: false
```

## Brute force
Calculate all possible combinations of the letters in s and search for t.

Time: O(N!)
Space: O(N!)

For each letter, we need a version of letters with the letter at the beginning resulting in factorial growth.
Any key: value store with grow at a similar rate.

## Possible solutions
- Convert s & t into strings and compare
    <em>Fail: duplicated characters aren't accounted for</em>

- Use a hashtable and keep a tally of the letters in s and their frequencies
    - Reverse the function using t and assert that the hashtable is empty
        - More space efficient but not clean
    - Build a replica using T and compare

## Solution
Choose solution #2

Time: O(N)
Space: O(N)

The mapping operations, although repeated twice, are constant (ie. 2) and linear.
The tally maps grow with inputs.

def tallystring function
    - Loop over s and build frequency map

get frequency maps for s and t
iterate over keys in tmap
    check exists in smap and check tally matches then delete key
check the hashmap is empty
