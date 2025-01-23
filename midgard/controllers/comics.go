package controllers

import (
	"midgard/entities"
	"net/http"

	"github.com/gin-gonic/gin"
)

var comics = []entities.Comic{
	/*
		this is the db (note we'll need to actually
		use a db otherwise caching will have little to zero
		effect since this is in-memory anway...)
	*/
	{ID: "1", Title: "X-Men", Author: "Robert RobRob", Condition: "NM"},
	{ID: "2", Title: "Uncanny X-Men", Author: "Philip Pattern", Condition: "M"},
	{ID: "3", Title: "Spiderman", Author: "Robert RobRob", Condition: "G"},
}

// getComics produces JSON payload of all comics in store
func GetComics(ctx *gin.Context) {
	ctx.IndentedJSON(http.StatusOK, comics)
}
