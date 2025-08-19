package backends

import (
	"balancer/result"
	"fmt"
	"os"
	"strings"
)

const backendsEnvVar = "BALANCER_BACKENDS"

func GetBackends() result.Result[BackendsInfo] {
	if rawBackends, ok := os.LookupEnv(backendsEnvVar); ok {
		backends := strings.Split(rawBackends, ";")
		return result.Ok(CreateBackendsInfo(backends))
	} else {
		return result.Err[BackendsInfo](fmt.Errorf("the %s environment variable is not set", backendsEnvVar))
	}
}
