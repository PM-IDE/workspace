package main

import (
	"balancer/backends"
	"fmt"
	"os"
)

func main() {
	urls := backends.GetBackendsUrls()
	if urls.IsErr() {
		fmt.Println(urls.Err().Error())
		os.Exit(-1)
	}

	StartServer(*urls.Ok())
}
