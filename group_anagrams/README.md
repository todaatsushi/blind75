# 49. Group Anagrams
[Link](https://leetcode.com/problems/group-anagrams/)

```
Given an array of strings strs, group the anagrams together. You can return the answer in any order.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

Constraints:
- 1 <= strs.length <= 104
- 0 <= strs[i].length <= 100
- strs[i] consists of lowercase English letters.
```

# Breakdown

# Examples

Input: strs = ["eat","tea","tan","ate","nat","bat"]
Output: [["bat"],["nat","tan"],["ate","eat","tea"]]

Input: strs = [""]
Output: [[""]]

Input: strs = ["a"]
Output: [["a"]]

## Brute force

Loop over every word in `strs` and compare to every known anagram. If the letter counts match, add to that anagram
group, else add to a new one.

Time: O(NMA)
Space: O(N)

Each loop (N) would need to cycle through each anagram group (M) and compare every letter of that word to the first of
each of those anagram groups (A).

The largest the anagrams could get is the size of `strs`.

## Solutions
1) Sort words and triage based on the results

Iterate over words in `strs` and sort (n log n ) on every word. The resulting words should be the exact same for each
anagram in the `strs`. We can then compare and add them to their own lists.

Time: O(N^^2 Log N)
Space: O(N)

Every word is sorted which is queried in a hash table and triaged. At the end, we loop and centralise into a results
list which also scales with the input.
