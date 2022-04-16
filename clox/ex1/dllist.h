#include <stdlib.h>
#include <string.h>
typedef struct StringNode{
    char *content;
    struct StringNode *prev;
    struct StringNode *next;
}str_node_t;

typedef struct StringList{
    str_node_t *head;
    str_node_t *tail;
}str_list_t;

void insert_str_node(str_list_t *list, char *new_content) {
    if(strncmp(new_content, "\0",1) == 0) return ;
    size_t str_len = sizeof(new_content)/sizeof(new_content[0]);
    str_node_t *node = (struct StringNode*)malloc(sizeof(struct StringNode));
    node->next, node->prev = NULL, NULL;
    node->content = (char*)malloc(sizeof(char) * str_len);
    snprintf(node->content, str_len, "%s", new_content);
    if(list->head == NULL){
        list->head = node;
        list->head->prev = NULL;
        list->tail = node;
        return ;
    }
    str_node_t *curr = list->head;
    while(curr->next != NULL){
        curr = curr->next;
    }
    curr->next = node;
    curr->next->prev = curr;
    list->tail = curr->next;
    return ;
}

str_node_t find_str_node(str_list_t *list, char *content_to_find) {
    
}

void delete_str_node(str_list_t *list, char *content_to_delete) {

}