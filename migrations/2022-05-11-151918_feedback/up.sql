CREATE TABLE
    feedback (
        feedback_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

        TEXT TEXT NOT NULL,
        rating SMALLINT NOT NULL,
        nps SMALLINT NOT NULL,

		created_at TIMESTAMP DEFAULT NOW(),

		user_id UUID NOT NULL,

        FOREIGN KEY(user_id) REFERENCES users(user_id) ON DELETE CASCADE
    );