#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

// Creates an orphan process to be reaped. Useful for testing.
int main() {
    pid_t pid;

    if ((pid = fork()) < 0) {
        perror("could not create a child process");
        exit(1);
    }

    if (pid > 0) {
        printf("%9s: pid=%06d ppid=%06d pgid=%06d\n", "parent", getpid(), getppid(), getpgid(0));
        sleep(10);
    } else {
        printf("%9s: pid=%06d ppid=%06d pgid=%06d\n", "child", getpid(), getppid(), getpgid(0));
        // Wait to be reparented.
        while (getppid() > 1) {
        }
        printf("%9s: pid=%06d ppid=%06d pgid=%06d\n", "child", getpid(), getppid(), getpgid(0));
    }

    return 0;
}
