/*
 * compile-command: "gcc -Wall -Werror -lcap -lseccomp contained.c -o contained"
 * This code is licensed under the GPLv3. You can find its text here:
 * https://www.gnu.org/licenses/gpl-3.0.en.html 
 */

#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <grp.h>
#include <pwd.h>
#include <sched.h>
#include <seccomp.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sys/capability.h>
#include <sys/mount.h>
#include <sys/prctl/h>
#include <sys/resource.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <sys/utsname.h>
#include <sys/wait.h>

// The issue is that this uses Linux specific libraries...
#include <linux/capability.h>
#include <linux/limits.h>

struct child_config {
    int argc;
    uid_t uid;
    int fd;
    char *hostname;
    char **argv;
    char *mount_dir;
};

<<capabilities>>

<<mounts>>

<<syscalls>>

<<resources>>

<<child>>

<<choose-hostname>>

int main(int argc, char **argv)
{
    struct child_config config = {0};
    int err = 0;
    int option = 0;
    int sockets[2] = {0};
    int last_optind = 0;

    while((option = getopt(argc, argv, "c:m:u:"))) {
        switch (option) {
        case 'c':

}

