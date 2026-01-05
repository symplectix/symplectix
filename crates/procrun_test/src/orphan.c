#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int orphan(int n) {
    pid_t child;
    int max_depth = 3;

    if ((child = fork()) < 0) {
        perror("could not create a child process");
        exit(1);
    }

    pid_t pid    = getpid();
    pid_t group  = getpgid(0);
    pid_t parent = getppid();

    if (child > 0) {
        if (n == 0) {
            fprintf(
                stdout,
                "Parent\tpid=%d\tgroup=%d\tparent=%d\tchild=%d\n",
                pid,
                group,
                parent,
                child
            );
            fflush(stdout);
        }

        // the first process is monitored by run.
        sleep(max_depth-n);
        exit(0);
    } else {
        if (n < max_depth) {
            orphan(n+1);
        }

        // Wait to be reparented.
        while (getppid() == parent) {
        }
        fprintf(
            stdout,
            "Orphan\tpid=%d\tgroup=%d\tparent=%d\tparent_before=%d\n",
            pid,
            group,
            getppid(),
            parent
        );
        fflush(stdout);
        exit(0);
    }
}

// Creates orphan/zombie process tree to be reaped.
int main() {
    orphan(0);
}
