# 206. Reverse Linked List
[Link](https://leetcode.com/problems/reverse-linked-list/)

```
Given the head of a singly linked list, reverse the list, and return the reversed list.

Constraints:
- The number of nodes in the list is the range [0, 5000].
- -5000 <= Node.val <= 5000
```

# Breakdown
Linked lists will have a head that leads to a collection of nodes. Each node has a value and also a `next` value which
holds a reference to the next node.

Each reference will need to point to the node before it which, it doesn't hold one of. The challenge comes from that
lack of reference.

## Solution
Use a temporary variable holding the 'next' node, which we can then use to hook up the previous node to the current one.
We continue using the current node and then moving to the next node until the next node is empty, which we then return.

Time: O(N)
Space: O(1)

We need to loop through the entire linked list but the varaiable storing the temp node never grows or shrinks due to the
size of the input.
