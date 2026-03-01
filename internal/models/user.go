package models

import "gorm.io/gorm"

type User struct {
	gorm.Model

	Email        string `gorm:"uniqueIndex;not null;size:255"`
	PasswordHash string `gorm:"not null"`
	Name         string `gorm:"size:120"`
}
