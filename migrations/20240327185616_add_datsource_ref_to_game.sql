PRAGMA foreign_keys=off;

ALTER TABLE games ADD COLUMN datasource_id INTEGER NOT NULL;
ALTER TABLE games RENAME TO _games_old;

CREATE TABLE games (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    regions VARCHAR NOT NULL,
    device BOOLEAN NOT NULL DEFAULT false,
    system_id INTEGER NOT NULL,
    parent_id INTEGER, complete BOOLEAN NOT NULL DEFAULT false, sorting SMALLINT NOT NULL DEFAULT 0, comment VARCHAR, bios BOOLEAN NOT NULL DEFAULT false, bios_id INTEGER REFERENCES games(id) ON DELETE CASCADE, external_id VARCHAR, jbfolder BOOLEAN NOT NULL DEFAULT false, playlist_id INTEGER REFERENCES romfiles(id) ON DELETE SET NULL,
    datsource_id INTEGER NOT NULL,
    FOREIGN KEY (system_id) REFERENCES systems(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES games(id) ON DELETE CASCADE,
    FOREIGN KEY (datsource_id) REFERENCES datsource(id),
    UNIQUE (name, system_id)
);

INSERT INTO games SELECT * FROM _games_old;


PRAGMA foreign_keys=on;
