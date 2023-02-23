// LeetCode 102. Binary Tree Level Order Traversal
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    vector<vector<int>> levelOrder(TreeNode* root) {
        if (root == nullptr) return {};

        vector<vector<int>> traversal_history;
        int layer = 0;

        preorder_traversal(root, traversal_history, layer);

        return traversal_history;
    }

    void preorder_traversal(TreeNode* root, vector<vector<int>> &history, int &layer) {
        if (root != nullptr) {
            vector<int> new_layer;
            if (layer >= history.size()) { history.push_back(new_layer); }
            history[layer].push_back(root->val);
            ++layer;
            preorder_traversal(root->left, history, layer);
            preorder_traversal(root->right, history, layer);
            --layer;
        }
    }
};
