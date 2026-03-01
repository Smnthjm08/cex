package main

import (
	"encoding/json"
	"log"
	"net/http"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/go-chi/cors"
	"github.com/smnthjm08/exchange/internal/connections"
	"github.com/smnthjm08/exchange/internal/env"
	"github.com/smnthjm08/exchange/internal/models"
)

type config struct {
	port  string
	dbURL string
}

func main() {

	config := config{
		port:  env.String("PORT", "5002"),
		dbURL: env.MustString("DATABASE_URL"),
	}

	connections.ConnectDatabase(config.dbURL)

	err := connections.DB.AutoMigrate(&models.User{})
	if err != nil {
		log.Fatal("migration failed:", err)
	}

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
			r.Post("/login", func(w http.ResponseWriter, r *http.Request) {
				w.Write([]byte("login endpoint"))
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

	log.Println("exchange backend starting 🚀")
	log.Printf("running at http://localhost:%s", config.port)
	log.Fatal(http.ListenAndServe(":"+config.port, r))
}
