#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <dirent.h>
#include <stdlib.h>
#include <string.h>

#define BUFSIZ 20

// get inode of current directory
ino_t get_inode(char*);
// print path
void print_path_to(ino_t);
// find name of a directory
void ino_to_name(ino_t, char*, int);

int main() {
  print_path_to(get_inode("."));
  printf("\n");
  return 0;
}

ino_t get_inode(char* path) {
  struct stat info;
  if (stat(path, &info) == -1) {
    fprintf(stderr, "Cannot stat %s\n.", path);
    exit(1);
  }
  return info.st_ino;
}

void ino_to_name(ino_t inode_to_find, char* name, int buf_len) {
  DIR* dir_ptr;
  struct dirent* entry;
  if ((dir_ptr = opendir(".")) == NULL) {
    perror(".");
    exit(1);
  }
  while ((entry = readdir(dir_ptr)) != NULL) {
    if (entry->d_ino == inode_to_find) {
      strncpy(name, entry->d_name, buf_len);
      name[BUFSIZ - 1] = '\0';
      closedir(dir_ptr);
      return;
    }
  }
  closedir(dir_ptr);
  fprintf(stderr, "Cannot find ino %d\n", inode_to_find);
}

void print_path_to(ino_t ino) {
  ino_t my_inode;
  char* name[BUFSIZ];
  if (get_inode("..") != ino) {
    chdir("..");
    ino_to_name(ino, name, BUFSIZ);
    my_inode = get_inode(".");
    print_path_to(my_inode);
    printf("/%s", name);
  }
}