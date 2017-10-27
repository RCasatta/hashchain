package main

import (
	"github.com/minio/sha256-simd"
	"fmt"
	"crypto/rand"
	"time"
)

func main() {

	current := make([]byte, 32)
	rand.Read(current)
	start := time.Now()

	for i := int64(0) ; i < 1000000000; i++ {
		if(i%1000000==0) {
			elapsed := time.Since(start).Nanoseconds()/1000

			fmt.Printf("%d %x %f\n", i, current, float64(i)/float64(elapsed))
		}
		result := sha256.Sum256([]byte(current))
		current = result[:];

	}



}

