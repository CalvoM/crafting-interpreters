#include <stdio.h>
#include "ex1/dllist.h"

void free_mem(str_list_t *list);
int main(int argc, char** argv) {
    str_list_t *mylist = malloc(sizeof(struct StringList));
    mylist->head = NULL;
    mylist->tail = NULL;
    insert_str_node(mylist, "Mojo World");
    insert_str_node(mylist, "Rusty");
    insert_str_node(mylist, "Shatter Star");
    insert_str_node(mylist, "The vanisher");
    insert_str_node(mylist, "I am lucky");
    printf("%s\n", mylist->head->content);
    printf("%s\n", mylist->head->next->content);
    printf("%s\n", mylist->head->next->next->content);
    free_mem(mylist);
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