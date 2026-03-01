package env

import (
	"log"
	"os"
)

func String(key, fallback string) string {
	if v, ok := os.LookupEnv(key); ok && v != "" {
		return v
	}
	return fallback
}

func MustString(key string) string {
	if v, ok := os.LookupEnv(key); ok && v != "" {
		return v
	}
	log.Fatalf("missing required env: %s", key)
	return ""
}
