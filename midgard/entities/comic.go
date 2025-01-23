package entities

// structure representing a comic book
type Comic struct {
	ID        string `json:"id"`
	Title     string `json:"title"`
	Author    string `json:"author"`
	Condition string `json:"condition"`
}
