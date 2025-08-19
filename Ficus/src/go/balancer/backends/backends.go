package backends

import (
	"fmt"
	"os"
	"strings"
)

const backendsEnvVar = "BALANCER_BACKENDS"

func GetBackends() (*BackendsInfo, error) {
	if rawBackends, ok := os.LookupEnv(backendsEnvVar); ok {
		backends := strings.Split(rawBackends, ";")
		return CreateBackendsInfo(backends), nil
	} else {
		return nil, fmt.Errorf("the %s environment variable is not set", backendsEnvVar)
	}
}
