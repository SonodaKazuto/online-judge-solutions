// LeetCode 590. N-ary Tree Postorder Traversal
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
    vector<int> postorder(Node* root) {
        vector<int> history;
        if (root != nullptr) {
            for (Node* child: root->children) {
                vector<int> child_history = postorder(child);
                history.insert(history.end(), child_history.begin(), child_history.end());
            }
            history.push_back(root->val);
        }
        return history;
    }
};
