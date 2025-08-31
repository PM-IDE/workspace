package backends

import (
	"balancer/result"
	"fmt"
	"os"
	"strings"
)

const backendsEnvVar = "BALANCER_BACKENDS"

func GetBackendsUrls() result.Result[[]string] {
	if rawBackends, ok := os.LookupEnv(backendsEnvVar); ok {
		backends := strings.Split(rawBackends, ";")
		return result.Ok[[]string](&backends)
	} else {
		return result.Err[[]string](fmt.Errorf("the %s environment variable is not set", backendsEnvVar))
	}
}
