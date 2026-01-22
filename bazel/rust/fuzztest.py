import argparse
import os
import tempfile
from pathlib import Path


def _tmpdir() -> Path:
    if tmpdir := os.getenv("TEST_TMPDIR"):
        return Path(tmpdir)
    return tempfile.TemporaryDirectory().name


def run(output_root: Path | None, fuzz_target: Path, fuzz_args: list[str]) -> None:
    """Execute fuzz_target from BUILD_WORKING_DIRECTORY."""
    if output_root is None:
        with tempfile.TemporaryDirectory(delete=False) as tmpdir:
            output_root = Path(tmpdir) / "fuzzing"

    fuzz_target = fuzz_target.resolve()
    fuzztest_name = fuzz_target.name.removesuffix("_fuzz_target")

    if bwd := os.getenv("BUILD_WORKING_DIRECTORY"):
        os.chdir(bwd)

    artifact_prefix = output_root / fuzztest_name / "artifact"
    artifact_prefix.mkdir(parents=True, exist_ok=True)

    os.execv(  # noqa: S606
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
    run(**vars(args))
