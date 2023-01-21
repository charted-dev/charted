DROP TABLE IF EXISTS audit_logs;
DROP TABLE IF EXISTS webhook_events;

CREATE TABLE IF NOT EXISTS `audit_logs`(
    ID UInt64,
    FiredAt DateTime64,
    Data JSON,
    Origin UInt64,
    OriginType enum('repo' = 0, 'org' = 1),
    Action enum(
        'repo.modify' = 0,
        'repo.starred' = 1,
        'repo.unstarred' = 2,
        'repo.push' = 3,
        'repo.pull' = 4,
        'repo.member_perm_update' = 5,
        'org.modify' = 6,
        'org.new_member' = 7,
        'org.updated_member' = 8,
        'org.kicked_member' = 9,
        'org.member_perm_update' = 10
    )
) ENGINE = MergeTree() PARTITION BY toYYYYMM(FiredAt) ORDER BY (Action, FiredAt, ID, Origin);

CREATE TABLE IF NOT EXISTS `webhook_events`(
    ID UInt64,
    FiredAt DateTime64,
    Data JSON,
    Origin UInt64,
    OriginType enum('repo' = 0, 'org' = 1),
    Action enum(
        'repo.modify' = 0,
        'repo.starred' = 1,
        'repo.unstarred' = 2,
        'repo.push' = 3,
        'repo.pull' = 4,
        'repo.member_perm_update' = 5,
        'org.modify' = 6,
        'org.new_member' = 7,
        'org.updated_member' = 8,
        'org.kicked_member' = 9,
        'org.member_perm_update' = 10
    )
) ENGINE = MergeTree() PARTITION BY toYYYYMM(FiredAt) ORDER BY (Action, FiredAt, ID, Origin);
