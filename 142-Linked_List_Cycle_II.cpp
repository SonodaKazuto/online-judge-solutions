// LeetCode 142. Linked List Cycle II
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    ListNode *detectCycle(ListNode *head) {
        vector<ListNode*> linked_list;
        ListNode *cur = head;

        while (cur != nullptr) {
            linked_list.push_back(cur);
            auto index = find(linked_list.begin(), linked_list.end(), cur->next);
            if (index != linked_list.end()) return linked_list.at(index - linked_list.begin());
            cur = cur->next;
        }
        
        return nullptr;
    }
};
