package main

import (
	"fmt"
	"midgard/controllers"

	"github.com/gin-gonic/gin"
)

func main() {
	initServer()
}

func initServer() {
	router := gin.Default()
	router.GET("/comics", controllers.GetComics)
	router.GET("/comics/:id", controllers.GetComic)
	router.Run("localhost:8080")
	fmt.Print("Controllers mapped, router initialized.")
}
