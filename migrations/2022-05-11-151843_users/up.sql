CREATE TABLE
    users (
        user_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

        name TEXT NOT NULL,
        surname TEXT NOT NULL,
        fiscal_number CHAR(16) NOT NULL,
        gender CHAR(1) NOT NULL,
        category VARCHAR(64) NOT NULL,

        email TEXT NOT NULL,
        phone_number TEXT NOT NULL,

        education TEXT,
        job TEXT,
        workplace TEXT,
        other_associations TEXT,

        birthday DATE NOT NULL,
        birthplace TEXT NOT NULL,
        birthplace_province CHAR(2) NOT NULL,
        birthplace_istat_code CHAR(5) NOT NULL,

        address TEXT NOT NULL,
        city TEXT NOT NULL,
        province CHAR(2) NOT NULL,
        zip_code CHAR(5) NOT NULL,
        istat_code CHAR(5) NOT NULL
    );