ALTER TABLE webhook_events ADD COLUMN success_rate UInt64;
ALTER TABLE webhook_events ADD COLUMN failure_rate UInt64;
