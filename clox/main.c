#include <stdio.h>
#include "ex1/dllist.h"

void free_mem(str_list_t *list);
int main(int argc, char** argv) {
    str_list_t *mylist = malloc(sizeof(struct StringList));
    mylist->head = NULL;
    mylist->tail = NULL;
    insert_str_node(mylist, "Mojo World");
    insert_str_node(mylist, "Rusty");
    insert_str_node(mylist, "Shatter Star 40 years.");
    insert_str_node(mylist, "The vanisher");
    insert_str_node(mylist, "I am lucky");
    str_node_t * node = find_str_node(mylist, "Mojo World ");
    if(node != NULL) {
        printf("Text found.\n");
    }
    print_list(mylist);
    free_mem(mylist);
}
