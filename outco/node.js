class Node {
	constructor(val) {
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
