package connections

import (
	"log"
	"sync"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

var (
	DB   *gorm.DB
	once sync.Once
)

func ConnectDatabase(dbUrl string) *gorm.DB {
	once.Do(func() {

		db, err := gorm.Open(postgres.Open(dbUrl), &gorm.Config{})
		if err != nil {
			log.Fatal("failed to connect database:", err)
		}

		DB = db
		log.Println("database connected!")
	})

	return DB
}
