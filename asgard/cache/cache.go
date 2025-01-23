package cache

// simple map to represent a cache holding generic items
import (
	"midgard/entities"
)

type ComicCache struct {
	// do we have type specific caches?
	// or keep it generic like {k:v}
	Comics []entities.Comic `json:Comics`
}
