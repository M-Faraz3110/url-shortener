CREATE TABLE urls
(
    id UUID PRIMARY KEY,
    user_id UUID,
    url TEXT NOT NULL,
    short_url TEXT NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_urls_urls ON urls (url);
CREATE INDEX idx_urls_short_url ON urls (short_url);
CREATE INDEX idx_urls_user_id ON urls (user_id);

