DROP TABLE IF EXISTS audit_logs;
DROP TABLE IF EXISTS webhook_events;

CREATE TABLE IF NOT EXISTS `audit_logs`(
    ID UInt64,
    FiredAt DateTime64,
    Data JSON,
    Origin UInt64,
    OriginType enum('repo', 'org'),
    Action enum(
        'repo.modify',
        'repo.starred',
        'repo.unstarred',
        'repo.push',
        'repo.pull',
        'repo.member_perm_update',
        'org.modify',
        'org.new_member',
        'org.updated_member',
        'org.kicked_member',
        'org.member_perm_update'
    )
) ENGINE = MergeTree() PARTITION BY toYYYYMM(FiredAt) ORDER BY (Action, FiredAt, ID, Origin);

CREATE TABLE IF NOT EXISTS `webhook_events`(
    ID UInt64,
    FiredAt DateTime64,
    Data JSON,
    Origin UInt64,
    OriginType enum('repo', 'org'),
    Action enum(
        'repo.modify',
        'repo.starred',
        'repo.unstarred',
        'repo.push',
        'repo.pull',
        'repo.member_perm_update',
        'org.modify',
        'org.new_member',
        'org.updated_member',
        'org.kicked_member',
        'org.member_perm_update'
    )
) ENGINE = MergeTree() PARTITION BY toYYYYMM(FiredAt) ORDER BY (Action, FiredAt, ID, Origin);
