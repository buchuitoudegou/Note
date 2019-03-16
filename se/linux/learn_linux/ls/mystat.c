#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <dirent.h>

void show_stat_info(char* filename, struct stat* buf) {
	printf("mode: %o\n", buf->st_mode);
	printf("links: %d\n", buf->st_nlink);
	printf("user: %d\n", buf->st_uid);
	printf("group: %d\n", buf->st_gid);
	printf("size:: %d\n", buf->st_size);
	printf("modtime: %d\n", buf->st_mtime);
	printf("inode: %d\n", buf->st_ino);
	printf("name: %s\n", filename);

	// DIR* dirptr;
	// dirptr = opendir(filename);
	// struct dirent* entry;
	// while ((entry = readdir(dirptr)) != NULL) {
	// 	printf("%s\n", entry->d_name);
	// }
}

int main(int argc, char const *argv[]) {
	struct stat info;
	if (argc > 1) {
		if (stat(argv[1], &info) != -1) {
			show_stat_info(argv[1], &info);
		} else {
			perror(argv[1]);
		}
	}
	return 0;
}