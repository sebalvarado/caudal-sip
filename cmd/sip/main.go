// Command sip runs the scheme core — the central system a central bank would
// operate. It currently does nothing but report its build; the ledger it will
// be built on is the next thing to land.
//
// Build order: docs/plan/core-build-order.md
package main

import (
	"fmt"
	"runtime/debug"
)

func main() {
	version := "dev"
	if info, ok := debug.ReadBuildInfo(); ok {
		for _, s := range info.Settings {
			if s.Key == "vcs.revision" {
				version = s.Value
				break
			}
		}
	}
	fmt.Printf("sip %s\n", version)
}
