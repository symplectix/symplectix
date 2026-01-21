import os
import sys
from pathlib import Path


def _tmpdir() -> str:
    if tmpdir := os.getenv("TEST_TMPDIR"):
        return tmpdir
    elif tmpdir := os.getenv("TMPDIR"):
        return tmpdir
    return "/tmp"


def _create_artifact_prefix_dir() -> str:
    tmpdir = _tmpdir()
    artifact_prefix = f"{tmpdir}/fuzzing/artifact/"
    os.makedirs(artifact_prefix, exist_ok=True)
    return artifact_prefix


if __name__ == "__main__":
    artifact_prefix = _create_artifact_prefix_dir()

    fuzz_target = sys.argv[1]
    fuzz_args = [
        Path(fuzz_target).name,
        f"-artifact_prefix={artifact_prefix}",
        *sys.argv[2:],
    ]
    os.execv(fuzz_target, fuzz_args)
