package main

import (
	"fmt"
	"log"
	"os"
)

func parse() {
	body, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatalf("unable to read file: %v", err)
	}
	fmt.Println(string(body))
}
