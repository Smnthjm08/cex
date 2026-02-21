package main

import (
	"encoding/json"
	"log"
	"net/http"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/go-chi/cors"
)

func main() {
	r := chi.NewRouter()

	// TODO this can be enabled later
	// r.Use(middleware.RequestID)
	r.Use(middleware.RealIP)
	r.Use(middleware.Recoverer)
	r.Use(middleware.Logger)
	r.Use(middleware.CleanPath)
	r.Use(middleware.AllowContentType("application/json"))

	// Basic CORS
	r.Use(cors.Handler(cors.Options{
		AllowedOrigins:   []string{"*"},
		AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
		AllowedHeaders:   []string{"Accept", "Authorization", "Content-Type", "X-CSRF-Token"},
		ExposedHeaders:   []string{"Link"},
		AllowCredentials: false,
		MaxAge:           300, // Maximum value not ignored by any of major browsers
	}))

	r.Get("/", func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("hello world!"))
	})

	r.Route("/api/v1", func(r chi.Router) {
		// health
		r.Get("/health", func(w http.ResponseWriter, r *http.Request) {
			resp := map[string]interface{}{
				"status": "ok",
				"time":   time.Now().UTC(),
			}

			w.Header().Set("Content-Type", "application/json")
			json.NewEncoder(w).Encode(resp)
		})

		// auth
		r.Route("/auth", func(r chi.Router) {
			r.Post("/signin", func(w http.ResponseWriter, r *http.Request) {
				w.Write([]byte("signin endpoint"))
			})

			r.Post("/signup", func(w http.ResponseWriter, r *http.Request) {
				w.Write([]byte("signup endpoint"))
			})

			r.Post("/forgot-password", func(w http.ResponseWriter, r *http.Request) {
				w.Write([]byte("forgot-password endpoint"))
			})

			r.Post("/reset-password", func(w http.ResponseWriter, r *http.Request) {
				w.Write([]byte("reset-password endpoint"))
			})
		})
	})

	r.NotFound(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(404)
		w.Write([]byte("route does not exist"))
	})
	r.MethodNotAllowed(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(405)
		w.Write([]byte("method is not valid"))
	})

	log.Println("Exchange backend starting 🚀")
	log.Fatal(http.ListenAndServe(":3000", r))
}
