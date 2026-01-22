#!/usr/bin/env python

import os
import subprocess
from typing import Literal


def git_status() -> Literal["clean", "dirty"]:
    """Return the working tree status."""
    status = subprocess.run(
        ["git", "status", "--porcelain"],
        encoding="utf-8",
        stdout=subprocess.PIPE,
        check=True,
    )
    if status.stdout:
        return "dirty"
    else:
        return "clean"


def git_revision() -> str:
    """Return the working tree revision.

    A revision refers to the id you can use as a parameter
    to reference an object in git (usually a commit).
    """
    status = subprocess.run(
        ["git", "rev-parse", "--short=10", "HEAD"],
        encoding="utf-8",
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        check=True,
    )
    return status.stdout


def github_run_number() -> str:
    """Return GITHUB_RUN_NUMBER or 0.

    A unique number for each run of a particular workflow in a repository.
    This number begins at 1 for the workflow's first run, and increments with each new run.
    This number does not change if you re-run the workflow run.

    https://docs.github.com/en/actions/learn-github-actions/contexts#github-context
    """
    num = os.getenv("GITHUB_RUN_NUMBER", "0")
    return f"r{num}"


if __name__ == "__main__":
    if bwd := os.getenv("BUILD_WORKING_DIRECTORY"):
        os.chdir(bwd)

    print("STABLE_GIT_STATUS", git_status())
