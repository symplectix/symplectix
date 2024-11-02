package run_test

import (
	"bufio"
	"bytes"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"os/exec"
	"runtime"
	"strconv"
	"strings"
	"syscall"
	"testing"

	"github.com/bazelbuild/rules_go/go/runfiles"
)

var (
	runRloc    = must(runfiles.Rlocation("trunk/process/run/cmd/run"))
	orphanRloc = must(runfiles.Rlocation("trunk/process/run/cmd/orphan"))
)

var run = &runCmd{
	loc: runRloc,
}

type runCmd struct {
	loc string
}

func (c *runCmd) Args(arg ...string) *exec.Cmd {
	cmd := exec.Command(c.loc, arg...)

	cmd.Env = []string{
		"RUN_LOG=run=trace,reaper=trace",
	}

	return cmd
}

func must[T any](t T, err error) T {
	if err != nil {
		panic(err)
	}

	return t
}

func executable(info fs.FileInfo) bool {
	return info.Mode()&0o111 != 0
}

func testRunfiles(t *testing.T, locs ...string) {
	t.Helper()
	if err := checkRunfiles(locs...); err != nil {
		t.Fatal(err)
	}
}

func checkRunfiles(locs ...string) error {
	errs := make([]error, len(locs))
	for _, loc := range locs {
		var path, mode error

		stat, err := os.Stat(loc)
		if errors.Is(err, os.ErrNotExist) {
			path = err
		}
		if !executable(stat) {
			mode = fmt.Errorf("not executable: %s", loc)
		}

		errs = append(errs, errors.Join(path, mode))
	}

	return errors.Join(errs...)
}

func TestRunfiles(t *testing.T) {
	testRunfiles(t, runRloc, orphanRloc)
}

func TestOrphan(t *testing.T) {
	cmd := run.Args(orphanRloc)

	var stderr bytes.Buffer
	cmd.Stderr = &stderr

	cmd.Start()
	runPid := cmd.Process.Pid

	defer func() {
		_ = cmd.Process.Signal(os.Interrupt)
	}()

	state, err := cmd.Process.Wait()
	if !state.Success() {
		t.Errorf("%v, %v", state, err)
	}

	var (
		pids    = make(map[string]string)
		scanner = bufio.NewScanner(&stderr)
	)
	for scanner.Scan() {
		before, after, found := strings.Cut(scanner.Text(), "\t")
		t.Logf("\nbefore:%s\nafter:%s", before, after)
		if !found {
			continue
		}

		var child, parent string
		for _, keyval := range strings.Split(after, "\t") {
			kv := strings.Split(keyval, "=")
			switch kv[0] {
			case "pid":
				child = kv[1]
			case "reparented":
				parent = kv[1]
			}
		}
		if child != "" && parent != "" {
			pids[child] = parent
		}
	}

	for child, parent := range pids {
		t.Logf("child:%s parent:%s run:%d", child, parent, runPid)
		if runtime.GOOS == "linux" && parent != strconv.Itoa(runPid) {
			// parent should be run if subreaper enabled.
			t.Error("subreaper disabled")
		}

		p, _ := os.FindProcess(must(strconv.Atoi(child)))
		// err should not be nil because the child should exit once run exited.
		if err := p.Signal(syscall.Signal(0)); err == nil {
			t.Errorf("process %s still running", child)
		}
	}
}

func TestMax(t *testing.T) {
	if got := max(0, 1); got != 1 {
		t.Errorf("want '1', but got %d:", got)
	}
	if got := max(-1, 1); got != 1 {
		t.Errorf("want '1', but got %d:", got)
	}
	if got := max(-1, -2); got != -1 {
		t.Errorf("want '-1', but got %d:", got)
	}
}

func TestMin(t *testing.T) {
	if got := min(0, 1); got != 0 {
		t.Errorf("want '0', but got %d:", got)
	}
	if got := min(-1, 0, 1); got != -1 {
		t.Errorf("want '-1', but got %d:", got)
	}
}
