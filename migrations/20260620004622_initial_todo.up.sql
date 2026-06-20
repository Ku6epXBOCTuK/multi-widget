CREATE TABLE
    activities (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        parent_id INTEGER REFERENCES activities (id) ON DELETE CASCADE,
        activity_type TEXT NOT NULL CONSTRAINT activities_type CHECK (
            activity_type IN ('project', 'stage', 'task', 'sub-task')
        ),
        title TEXT NOT NULL,
        description TEXT,
        status TEXT NOT NULL DEFAULT 'pending' CONSTRAINT activities_status CHECK (status IN ('pending', 'in-progress', 'done')),
        url TEXT,
        is_system INTEGER NOT NULL DEFAULT 0,
        created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE TRIGGER ts_activities_updated_at AFTER
UPDATE ON activities BEGIN
UPDATE activities
SET
    updated_at = CURRENT_TIMESTAMP
WHERE
    id = NEW.id;

END;