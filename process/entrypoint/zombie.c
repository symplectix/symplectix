#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main() {
    pid_t pid;

    if ((pid = fork()) < 0) {
        perror("could not create a child process");
        exit(1);
    }

    if (pid > 0) {
        printf("%9s: pid=%d ppid=%d pgid=%d\n", "parent", getpid(), getppid(), getpgid(0));
        sleep(100);
    } else {
        printf("%9s: pid=%d ppid=%d pgid=%d\n", "child", getpid(), getppid(), getpgid(0));
        exit(0);
    }

    return 0;
}
