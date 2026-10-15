# Database

First, look into the schemas of similar projects to get an initial idea of what the schema could or should look like.

## calibre-web
Uses a SQLite database with a schema that should be compatible with calibre. You can use calibre and calibre-web on the same library with little to no issues, maybe just with a slight limitation on custom columns the user created in calibre. 

The repository stores a sample database file [here](https://github.com/janeczku/calibre-web/blob/master/library/metadata.db).

## calibre
Uses a SQLite database with a schema that should be compatible with calibre-web.

Seems like calibre doesn't use schemas directly and instead defines its migrations in Python [here](https://github.com/kovidgoyal/calibre/blob/master/src/calibre/db/schema_upgrades.py).

## Audiobookshelf
Uses a SQLite database. Similarly to calibre, doesn't use direct migrations but JavaScript instead [here](https://github.com/advplyr/audiobookshelf/blob/master/server/utils/migrations/dbMigration.js).

## BookOrbit
Uses a separate postgreSQL database in its own Docker container. 

Defines migrations in proper SQL files [here](https://github.com/bookorbit/bookorbit/tree/main/server/src/db/migrations).
