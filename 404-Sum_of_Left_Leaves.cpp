// LeetCode 404. Sum of Left Leaves
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
    int sumOfLeftLeaves(TreeNode* root) {
        if (root == nullptr) return 0;
        TreeNode* left = root->left;
        TreeNode* right = root->right;

        if (left == nullptr) return sumOfLeftLeaves(right);
        if (left->left == nullptr && left->right == nullptr) return left->val + sumOfLeftLeaves(right);
        return sumOfLeftLeaves(left) + sumOfLeftLeaves(right);
    }
};
