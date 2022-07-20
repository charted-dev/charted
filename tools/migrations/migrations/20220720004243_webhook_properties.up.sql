ALTER TABLE webhooks ADD successful boolean;
ALTER TABLE webhooks ADD failed boolean;
ALTER TABLE webhooks ADD response_payload text;
ALTER TABLE webhooks ADD event text;
