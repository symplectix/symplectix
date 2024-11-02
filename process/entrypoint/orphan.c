#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

pid_t pid;

static void sigdown(int signo) {
    printf("%6d: pid=%d ppid=%d pgid=%d signal=%d\n", pid, getpid(), getppid(), getpgid(0), signo);
    // exit(0);
}

// Creates an orphan process to be reaped. Useful for testing.
int main() {
    if ((pid = fork()) < 0) {
        perror("could not create a child process");
        exit(1);
    }

    if (pid == 0) {
        printf("%6d: pid=%d ppid=%d pgid=%d\n", pid, getpid(), getppid(), getpgid(0));

        if (sigaction(SIGINT, &(struct sigaction){.sa_handler = sigdown}, NULL) < 0)
            return 1;
        if (sigaction(SIGTERM, &(struct sigaction){.sa_handler = sigdown}, NULL) < 0)
            return 2;

        while (getppid() > 1) {
        }

        printf("%6d: pid=%d ppid=%d pgid=%d\n", pid, getpid(), getppid(), getpgid(0));
        sleep(15);
        return 0;
    }

    printf("%6d: pid=%d ppid=%d pgid=%d\n", pid, getpid(), getppid(), getpgid(0));
    sleep(5);
    return 0;
}
