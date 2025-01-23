package time

import (
	"fmt"
	"math/rand"
	"time"
)

func SimulateDbLatency() {
	// very arbitrary decent db latency ~5-10ms
	r := rand.Intn(10)
	time.Sleep(time.Duration(r) * time.Millisecond)
	fmt.Print("slept for: ", r, "ms\n")
}
