package main

import (
	"balancer/backends"
	"fmt"
	"os"
)

func main() {
	res := backends.GetBackends()
	if res.IsErr() {
		fmt.Println(res.Err().Error())
		os.Exit(-1)
		return
	}

	fmt.Println(res.Ok())
}
