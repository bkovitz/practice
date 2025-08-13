/*
Given a binary tree, the task is to check if it is a Sum Tree. 
A Sum Tree is a Binary Tree where the value of a node is equal to the sum 
of the nodes present in its left subtree and right subtree. An empty tree is 
Sum Tree and the sum of an empty tree can be considered as 0. A leaf node is also considered a Sum Tree.

Example:


							26
    10									3
 4			6						 				3


1.) Understand
2.) Diagram
3.) Pseudocode
4.) Code


parent
[]

parent: 26 True
10+3+4+6+0+3

parent: 10 
4+6 True

parent: 3
0+3 True



Output: True

4+6 = 10
3+0 = 3
10+4+6+3+3=26

      0
  0			1

Output: False

*/

class Node {
	construstor(val) {
    this.val = val
    this.left = null;
    this.right = null;
  }
}


const isSumTree = (root) => {
  if (!root) {
    return true;
  }
  
  const dfs = (node) => {
    if (!node) {
      return true;
    }

    if (dfs(node.left) && dfs(node.right)) {
			children = 0;
		  if (node.left)
        children += node.left.val * 2;
      if (node.right)
        children += node.right.val * 2;
      return node.val == children;
    } else
      return false;
  }
  
  return dfs(root);
};

