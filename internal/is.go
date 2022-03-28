package internal

import (
	"io/ioutil"
	"os"
	"os/exec"
	"runtime"
	"strings"
)

/* cached value cuz why not >:c */
var isDocker *bool
var isKube *bool
var isRoot *bool

// Docker returns if Tsubaki is running under a Docker container.
// This package is a Go port from the NPM package `is-docker` by
// Sindre Sorhus. https://npm.im/is-docker
func Docker() bool {
	if isDocker == nil {
		value := hasDockerEnv() || hasDockerCGroup()
		isDocker = &value

		return value
	}

	return *isDocker
}

// Kubernetes returns if we are running on a Kubernetes pod. Most
// likely from the Noelware Helm Charts.
// This package is a Go port from the NPM package `is-kubernetes`
// by ntfs32 on GitHub. https://www.npmjs.com/package/is-kubernetes
func Kubernetes() bool {
	if isKube == nil {
		value := hasKubeEnv() || hasClusterDns() || hasServiceAccountFile()
		isKube = &value

		return value
	}

	return *isKube
}

// Root checks if (on windows) that it is using the Administrator Command Prompt
// or on a *NIX system that it is on the `root` account or using the "0" uid.
//
// This package is a Go port from the NPM package `is-root` and `is-admin` by
// Sindre Sorhus. https://npm.im/is-root | https://npm.im/is-admin
func Root() bool {
	if runtime.GOOS == "windows" {
		cmd := exec.Command("fsutil", "dirty", "query", os.Getenv("systemdrive"))
		_, err := cmd.Output()
		var value = err == nil
		if !value {
			value = testFltmc()
		}

		isRoot = &value
		return value
	}

	if isRoot == nil {
		value := os.Getuid() == 0
		isRoot = &value
	}

	return *isRoot
}

func hasDockerEnv() bool {
	_, err := os.Stat("/.dockerenv")
	return err == nil
}

func hasDockerCGroup() bool {
	contents, err := ioutil.ReadFile("/proc/self/cgroup")
	if err != nil {
		return false
	}

	return strings.Contains(string(contents), "docker")
}

func hasKubeEnv() bool {
	return os.Getenv("KUBERNETES_SERVICE_HOST") != ""
}

func hasServiceAccountFile() bool {
	var hasServiceAccountToken = false
	var hasServiceAccountNS = false

	_, err := os.Stat("/run/secrets/kubernetes.io/serviceaccount/token")
	if err == nil {
		hasServiceAccountToken = true
	}

	_, err = os.Stat("/run/secrets/kubernetes.io/serviceaccount/token")
	if err == nil {
		hasServiceAccountNS = true
	}

	return hasServiceAccountToken && hasServiceAccountNS
}

func hasClusterDns() bool {
	contents, err := ioutil.ReadFile("/etc/resolv.conf")
	if err != nil {
		return false
	}

	return strings.Contains(string(contents), "cluster.local")
}

func testFltmc() bool {
	cmd := exec.Command("fltmc")
	_, err := cmd.Output()

	return err == nil
}
