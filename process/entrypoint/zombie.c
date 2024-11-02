#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main() {
    pid_t pid;

    if ((pid = fork()) < 0) {
        perror("could not create a child process");
        exit(1);
    }

    if (pid == 0) {
        printf("%6d: pid=%d ppid=%d pgid=%d\n", pid, getpid(), getppid(), getpgid(0));
        exit(1);
    }

    printf("%6d: pid=%d ppid=%d pgid=%d\n", pid, getpid(), getppid(), getpgid(0));

    // sleep to observe the zombie child.
    sleep(30);
    return 0;
}
