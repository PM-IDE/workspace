package main

import (
	"balancer/backends"
	"fmt"
	"os"
)

func main() {
	backendsInfo, err := backends.GetBackends()
	if err != nil {
		fmt.Println(err.Error())
		os.Exit(-1)
		return
	}

	fmt.Println(backendsInfo)
}
