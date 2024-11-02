#!/usr/bin/env bash

# A "revision" refers to the id you can use as a parameter
# to reference an object in git (usually a commit).
git_revision() {
    git rev-parse HEAD 2>/dev/null
}
echo "STABLE_GIT_REVISION $(git_revision)"

# The working tree status.
git_status() {
    if [ -z "$(git status --porcelain)" ]; then
        echo clean
    else
        echo dirty
    fi
}
echo "STABLE_GIT_STATUS $(git_status)"

# A unique build id for CI.
github_build_id() {
    # Github Actions environment variables for stamping.
    # https://docs.github.com/en/actions/learn-github-actions/contexts#github-context

    # A unique number for each workflow run within a repository.
    # This number does not change if you re-run the workflow run.
    local run_id="${GITHUB_RUN_ID:=0}"

    # A unique number for each run of a particular workflow in a repository.
    # This number begins at 1 for the workflow's first run, and increments with each new run.
    # This number does not change if you re-run the workflow run.
    local run_number="${GITHUB_RUN_NUMBER:=0}"

    # A unique number for each attempt of a particular workflow run in a repository.
    # This number begins at 1 for the workflow run's first attempt, and increments with each re-run.
    local run_attempt="${GITHUB_RUN_ATTEMPT:=0}"

    echo "${run_id}-${run_number}-${run_attempt}"
}
echo "STABLE_GITHUB_BUILD_ID $(github_build_id)"
