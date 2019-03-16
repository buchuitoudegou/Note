#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <dirent.h>
#include <stdlib.h>
#include <string.h>
#include <pwd.h>
#include <grp.h>
#include <time.h>

void do_ls(char* dirname);
void mode_to_letters(int mode, char str[]);
char* uid_to_name(uid_t);
char* gid_to_name(gid_t);
void show_stat_info(char*, struct stat*);
void do_stat(char*);

int main(int ac, char const* argv[]) {
	if (ac == 1) {
		do_ls(".");
	} else {
		while (-- ac) {
			do_ls(argv[1]);
		}
	}
	return 0;
}


/*
 * struct passwd {
 * char* pw_name; // username
 * char* pw_passwd; // password
 * _uid_t pw_uid; // user id
 * _gid_t pw_gid; // group id
 * char* pw_gecow; // real name
 * char* pw_dir; // home directory
 * char* pw_shell; // shell program
 * }
*/
char* uid_to_name(uid_t uid) {
	struct passwd* pw_ptr;
	char numstr[10];
	if ((pw_ptr = getpwuid(uid)) != NULL) {
		return pw_ptr->pw_name;
	} else {
		sprintf(numstr, "%d", uid);
		return numstr;
	}
}
char* gid_to_name(gid_t gid) {
	struct group* grp_ptr;
	char numstr[10];
	if ((grp_ptr = getgrgid(gid)) != NULL) {
		return grp_ptr->gr_name;
	} else {
		sprintf(numstr, "%d", gid);
		return numstr;
	}
}

void do_stat(char* filename) {
	struct stat info;
	if (stat(filename, &info) != -1) {
		show_stat_info(filename, &info);
	} else {
		perror(filename);
	}
}

void show_stat_info(char* filename, struct stat* info) {
	char modestr[11] = {0};
	mode_to_letters(info->st_mode, modestr);
	printf("%s ", modestr);
	printf("%4d ", (int)info->st_nlink);
	printf("%-8s ", uid_to_name(info->st_uid));
	printf("%-8s ", gid_to_name(info->st_gid));
	printf("%8ld ", (long)info->st_size);
	printf("%.12s ", 4 + ctime(&info->st_mtime));
	printf("%s\n", filename);
}

void mode_to_letters(int mode, char str[]) {
	strcpy(str, "----------");
	if (S_ISDIR(mode)) str[0] = 'd';
	if (S_ISCHR(mode)) str[0] = 'c';
	if (S_ISBLK(mode)) str[0] = 'b';

	if (mode & S_IRUSR) str[1] = 'r';
	if (mode & S_IWUSR) str[2] = 'w';
	if (mode & S_IXUSR) str[3] = 'x';

	if (mode & S_IRGRP) str[4] = 'r';
	if (mode & S_IWGRP) str[5] = 'w';
	if (mode & S_IXGRP) str[6] = 'x';

	if (mode & S_IROTH) str[7] = 'r';
	if (mode & S_IWOTH) str[8] = 'w';
	if (mode & S_IXOTH) str[9] = 'x';
}

void do_ls(char* dirname) {
	DIR* dir_ptr;
	struct dirent* entry;
	if ((dir_ptr = opendir(dirname)) == NULL) {
		fprintf(stderr, "ls1 cannot open %s\n", dirname);
		exit(1);
	}
	while ((entry = readdir(dir_ptr)) != NULL) {
		do_stat(entry->d_name);
	}
	closedir(dir_ptr);
}