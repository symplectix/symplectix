#!/usr/bin/env bash

echo "STABLE_GIT_COMMIT $(git rev-parse HEAD 2>/dev/null)"

# Environment variables for stamping.
# https://docs.github.com/en/actions/learn-github-actions/contexts#github-context
#
# A unique number for each workflow run within a repository.
# This number does not change if you re-run the workflow run.
echo "STABLE_GITHUB_RUN_ID ${GITHUB_RUN_ID:=0}"

# A unique number for each run of a particular workflow in a repository.
# This number begins at 1 for the workflow's first run, and increments with each new run.
# This number does not change if you re-run the workflow run.
echo "STABLE_GITHUB_RUN_NUMBER ${GITHUB_RUN_NUMBER:=0}"

# A unique number for each attempt of a particular workflow run in a repository.
# This number begins at 1 for the workflow run's first attempt, and increments with each re-run.
# echo "GITHUB_RUN_ATTEMPT ${GITHUB_RUN_ATTEMPT:=0}"
