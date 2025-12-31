#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int orphan(int n) {
    pid_t child;
    int max_depth = 20;

    if ((child = fork()) < 0) {
        perror("could not create a child process");
        exit(1);
    }

    if (child > 0) {
        pid_t pid    = getpid();
        pid_t group  = getpgid(0);
        pid_t parent = getppid();

        fprintf(
            stderr,
            "\tdepth=%d\tpid=%d\tgroup=%d\tparent=%d\tchild=%d\n",
            n,
            pid,
            group,
            parent,
            child
        );
        fflush(stderr);

        // the first process is monitored by run.
        sleep(max_depth-n);

        exit(n);
    } else {
        pid_t pid    = getpid();
        pid_t group  = getpgid(0);
        pid_t parent = getppid();

        if (n < max_depth) {
            orphan(n+1);
        }

        // Wait to be reparented.
        while (n == max_depth && getppid() == parent) {
        }
        fprintf(
            stderr,
            "\tdepth=%d\tpid=%d\tgroup=%d\treparented=%d\tparent_before=%d\n",
            n,
            pid,
            group,
            getppid(),
            parent
        );
        fflush(stderr);

        exit(n);
    }
}

// Creates orphan/zombie process tree to be reaped.
int main() {
    orphan(0);
}
