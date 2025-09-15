"""Run 'ruff check'."""

import os
import sys

import tools.ruff

if __name__ == "__main__":
    os.chdir(os.getenv("BUILD_WORKING_DIRECTORY", "."))
    tools.ruff.execv(*sys.argv[1:])
