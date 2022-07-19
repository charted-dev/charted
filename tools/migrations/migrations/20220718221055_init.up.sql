CREATE TABLE IF NOT EXISTS charted.audit_logs(
    id bigint PRIMARY KEY,
    origin_id bigint,
    origin_type text, /* 'repo' or 'organization */
    data text, /* json object since cassandra doesn't support json data types */
    fired_at timestamp,
    action text /* action enum string - https://charts.noelware.org/docs/server/features/audit-logs#actions */
);

CREATE TABLE IF NOT EXISTS charted.webhooks(
    id bigint PRIMARY KEY,
    origin_id bigint,
    origin_type text, /* 'repo', 'organization', 'user' */
    data text, /* dynamic data that was sent from server -> webhooks service */
    fired_at timestamp,
    response_code int /* response code */
);
