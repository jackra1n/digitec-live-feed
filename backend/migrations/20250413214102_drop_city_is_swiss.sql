BEGIN;

-- Drop the redundant isSwiss column from the CanonicalCity table
ALTER TABLE "CanonicalCity"
DROP COLUMN IF EXISTS "isSwiss";

COMMIT;
