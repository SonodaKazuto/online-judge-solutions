// LeetCode 141. Linked List Cycle
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
#include <vector>
class Solution {
public:
    bool hasCycle(ListNode *head) {
        if (head == nullptr) { return false; }

        ListNode *cur = head;
        vector<ListNode*> node_list;
        while (true) {
            if (cur == nullptr) { break; }
            auto index = std::find(node_list.begin(), node_list.end(), cur);
            if (index != node_list.end()) { return true; }
            node_list.push_back(cur);
            cur = cur->next;
        }

        return false;
    }
};
