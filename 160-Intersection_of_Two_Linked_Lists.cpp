// LeetCode 160. Intersection of Two Linked Lists
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
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        ListNode *cur_node = headA;
        vector<ListNode*> node_list;
        while (cur_node != nullptr) {
            node_list.push_back(cur_node);
            cur_node = cur_node-> next;
        }
        
        cur_node = headB;
        while (cur_node != nullptr) {
            auto index = std::find(node_list.begin(), node_list.end(), cur_node);
            if (index != node_list.end()) { return cur_node; }
            cur_node = cur_node->next;
        }

        return nullptr;
    }
};
