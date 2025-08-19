package main

import (
	"fmt"
	"os"
)

func main() {
	backendsInfo, err := getBackends()
	if err != nil {
		fmt.Println(err.Error())
		os.Exit(-1)
		return
	}

	fmt.Println(backendsInfo)
}
