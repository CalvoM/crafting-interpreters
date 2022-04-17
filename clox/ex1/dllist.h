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
    size_t str_len = strlen(new_content) + 1;
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

str_node_t* find_str_node(str_list_t *list, char *content_to_find) {
   str_node_t *curr = list->head;
   size_t str_len = strlen(content_to_find) + 1;
   if(strncmp(curr->content, content_to_find, str_len) == 0)  return curr;
   while(curr->next != NULL) {
       if(strncmp(curr->content, content_to_find, str_len) == 0){
           return curr;
       }
        curr = curr->next;
   }
   return NULL;
}

void delete_str_node(str_list_t *list, char *content_to_delete) {
    str_node_t * del_node = find_str_node(list, content_to_delete);
    if(del_node == NULL) return ;
    str_node_t *next = del_node->next;
    del_node->prev->next = next;
    free(del_node);
}

void free_mem(str_list_t *list) {
    str_node_t * node = list->head;
    while(node->next != NULL) {
        free(node->content);
        node = node->next;
        free(node->prev);
    }
    free(node->content);
    free(node);
    free(list);
}

void print_list(str_list_t *list) {
    str_node_t * node = list->head;
    while(node->next != NULL) {
        printf("%s\n", node->content);
        node = node->next;
    }
    printf("%s\n", node->content);

}