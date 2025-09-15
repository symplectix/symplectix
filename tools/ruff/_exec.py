import os
import sysconfig
from pathlib import Path
from typing import Final, NoReturn

from runfiles import Runfiles


def _runfiles() -> Runfiles:
    if rfs := Runfiles.Create():
        return rfs
    err = "Runfiles.Create failed"
    raise RuntimeError(err)


_rfs: Final[Runfiles] = _runfiles()

_RUFF_NOT_FOUND = FileNotFoundError("'ruff' not found")
_PYPROJECT_NOT_FOUND = FileNotFoundError("'pyproject.toml' not found")


def _pyproject() -> Path:
    if p := _rfs.Rlocation("symplectix/pyproject.toml"):
        return Path(p).resolve()
    raise _PYPROJECT_NOT_FOUND


def _ruff() -> Path:
    if fp := _rfs.Rlocation("rules_python++pip+pypi_313_ruff/bin/ruff"):
        return Path(fp).resolve()
    if ruff_extracted_whl_files := os.getenv("RUFF_EXTRACTED_WHL_FILES"):
        ruff = f"ruff{sysconfig.get_config_var('EXE')}"
        for fp in map(Path, ruff_extracted_whl_files.split(" ")):
            if fp.name == ruff and os.access(fp.as_posix(), os.F_OK | os.X_OK):
                return fp.resolve()
    raise _RUFF_NOT_FOUND


def execv(*args: str) -> NoReturn:
    """Run ruff executable found in runfiles."""
    ruff = _ruff()
    pyproject = _pyproject()
    os.execv(ruff, ["ruff", "--config", pyproject, *list(args)])  # noqa: S606
