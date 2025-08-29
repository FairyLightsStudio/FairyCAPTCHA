-- Enable the pgcrypto extension if not already enabled, to use gen_random_uuid()
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Table to store the details of a Proof-of-Work challenge (the "exam paper").
-- This data is transient and used to validate a client's attempt to solve a challenge.
CREATE UNLOGGED TABLE exam_sessions (
    -- Unique identifier for the session.
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- The public access key ID of the service that initiated the challenge.
    service_access_key_id VARCHAR(64) NOT NULL,

    -- A secret key for this specific session, sent to the client to authenticate the submission of the solution.
    session_secret VARCHAR(64) NOT NULL,

    -- The challenge string that the client needs to solve.
    challenge TEXT NOT NULL,

    -- The difficulty of the challenge (e.g., number of leading zeros).
    difficulty SMALLINT NOT NULL,

    -- The timestamp when this challenge expires and is no longer valid to be solved.
    expires_at TIMESTAMPTZ NOT NULL,

    -- Optional: The action this CAPTCHA is protecting (e.g., 'login', 'comment').
    action VARCHAR(255),

    -- Metadata about the client.
    -- client_ip INET,
    -- user_agent TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Table to store the proof tokens generated after a challenge is successfully solved.
-- These tokens are single-use; they are deleted from the table immediately after being verified.
CREATE UNLOGGED TABLE work_proof_tokens (
    -- Unique identifier for the proof.
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Reference to the original exam session. If the session is deleted, the proof is also deleted.
    -- exam_session_id UUID NOT NULL REFERENCES exam_sessions(id) ON DELETE CASCADE,

    -- The short-lived, unique proof token given to the client.
    proof_token TEXT NOT NULL UNIQUE,

    -- The timestamp when this proof token expires.
    expires_at TIMESTAMPTZ NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create an index on 'proof_token' for fast lookups during verification.
CREATE INDEX idx_work_proof_tokens_on_proof_token ON work_proof_tokens (proof_token);

-- Create an index on 'service_access_key_id' in exam_sessions for quick filtering.
CREATE INDEX idx_exam_sessions_on_service_access_key_id ON exam_sessions (service_access_key_id);

-- Create an index on 'expires_at' on both tables to efficiently clean up expired records.
CREATE INDEX idx_exam_sessions_on_expires_at ON exam_sessions (expires_at);
CREATE INDEX idx_work_proof_tokens_on_expires_at ON work_proof_tokens (expires_at);
