import argparse
import os
from pathlib import Path


def _tmpdir() -> Path:
    if tmpdir := os.getenv("TEST_TMPDIR"):
        return Path(tmpdir)
    elif tmpdir := os.getenv("TMPDIR"):
        return Path(tmpdir)
    return Path("/tmp")


def exec(output_root: Path | None, fuzz_target: Path, fuzz_args: list[str]) -> None:
    if output_root is None:
        output_root = _tmpdir() / "fuzzing"
    fuzz_target = fuzz_target.resolve()
    fuzztest_name = fuzz_target.name.removesuffix("_fuzz_target")
    artifact_prefix = output_root / fuzztest_name / "artifact"
    artifact_prefix.mkdir(parents=True, exist_ok=True)

    if bwd := os.getenv("BUILD_WORKING_DIRECTORY"):
        os.chdir(bwd)

    os.execv(
        fuzz_target,
        [
            fuzztest_name,
            f"-artifact_prefix={artifact_prefix}/",
            *fuzz_args,
        ],
    )


if __name__ == "__main__":
    p = argparse.ArgumentParser()
    p.add_argument("--output_root", type=Path)
    p.add_argument("fuzz_target", type=Path)
    p.add_argument("fuzz_args", nargs="*")
    args = p.parse_args()
    exec(**vars(args))
