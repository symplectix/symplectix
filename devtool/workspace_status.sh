#!/usr/bin/env bash

# The working tree status.
git_status() {
    if [ -z "$(git status --porcelain)" ]; then
        echo clean
    else
        echo dirty
    fi
}
echo "STABLE_GIT_STATUS $(git_status)"

# A "revision" refers to the id you can use as a parameter
# to reference an object in git (usually a commit).
git_revision() {
    git rev-parse --short=10 HEAD 2>/dev/null
}

# A unique build id for CI.
github_run_number() {
    # Github Actions environment variables for stamping.
    # https://docs.github.com/en/actions/learn-github-actions/contexts#github-context
    #
    # A unique number for each run of a particular workflow in a repository.
    # This number begins at 1 for the workflow's first run, and increments with each new run.
    # This number does not change if you re-run the workflow run.
    echo "r${GITHUB_RUN_NUMBER:=0}"
}

echo "STABLE_VERSION $(github_run_number).$(git_revision)"
