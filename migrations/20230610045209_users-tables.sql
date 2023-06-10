-- Add migration script here
CREATE TABLE IF NOT EXISTS
      users(
        id TEXT NOT NULL UNIQUE PRIMARY KEY,
        firstname VARCHAR(128) NOT NULL,
        lastname VARCHAR(128),
        email VARCHAR(128) UNIQUE NOT NULL,
        password VARCHAR(128) NOT NULL,
        avatar VARCHAR(128),
        createdAt TIMESTAMP DEFAULT NOW(),
        updatedAt TIMESTAMP DEFAULT NOW()
      );