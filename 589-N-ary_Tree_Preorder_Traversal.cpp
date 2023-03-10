// LeetCode 589. N-ary Tree Preorder Traversal
/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};
*/

class Solution {
public:
    vector<int> preorder(Node* root) {
        if (root == nullptr) return {};

        vector<int> traversal;

        child_traversal(root, traversal);

        return traversal;
    }

    void child_traversal(Node* root, vector<int> &traversal) {
        traversal.push_back(root->val);

        for (auto child = root->children.begin(); child != root->children.end(); child++) {
            child_traversal(*child, traversal);
        }
    }
};
