# booty
Acquiring fundamentals
in Rust.

## Initial Plan
- Simple api (go/gin) (midgard)
- Db is either csv/flatfile
- Simple web gui -> CRUD
- Cache service (asgard)


## API Requirements
- /comics
GET, POST - add a new comic and retrieve comics.
- /comics/:id
GET - get a comic by its id.

Each use json format.

## Running Commentary / Issue Tracker
- Writing cache in Rust for performance.
- Decide to recompute cache values externally
to avoid cache stampedes.