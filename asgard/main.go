package main

import (
	"booty/midgard/controllers"
	"fmt"

	"github.com/gin-gonic/gin"
)

func main() {

}

func initCache() {
	router := gin.Default()
	router.GET("/comics", controllers.GetComics)
	router.GET("/comics/:id", controllers.GetComic)
	router.Run("localhost:8080")
	fmt.Print("Controllers mapped, router initialized.")
}

/*
	I will implement the cache-aside pattern.
	- API check cache service for data
	if the data is not in the cache it loads
	from db and then saves it to the cache.
	Else retrieves data directly from cache.

	- If data is written to db the cache must
	be resynchronized.

*/
