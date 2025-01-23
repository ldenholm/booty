package main

import (
	"midgard/controllers"

	"github.com/gin-gonic/gin"
)

func main() {
	router := gin.Default()
	router.GET("/comics", controllers.GetComics)
	router.Run("localhost:8080")
}
