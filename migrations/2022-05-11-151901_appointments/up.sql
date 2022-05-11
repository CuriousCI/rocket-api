CREATE TABLE
    appointments (
        appointment_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

        at DATE NOT NULL,

        user_id UUID,

        picture_consent BOOLEAN DEFAULT FALSE,
        analysis_consent BOOLEAN DEFAULT FALSE,
        data_consent BOOLEAN NOT NULL,

        analysis TEXT,
        picture TEXT,

        created_at TIMESTAMP DEFAULT NOW(),
        confirmed_at TIMESTAMP,
        token UUID DEFAULT uuid_generate_v4(),

        FOREIGN KEY(user_id) REFERENCES users(user_id) ON DELETE CASCADE
    );