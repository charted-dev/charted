SET allow_experimental_object_type = 1;
CREATE TABLE IF NOT EXISTS "charted.audit_logs"(
  FiredAt DateTime64,
  ID UInt64,
  Action enum('repo.modify', 'repo.starred', 'repo.unstarred', 'repo.pull', 'repo.push', 'org.modify', 'org.new_member', 'org.updated_member', 'org.remove_member'),
  Data JSON,
  OriginID UInt64,
  OriginType enum('repo', 'org')
) ENGINE = MergeTree() PARTITION BY toYYYYMM(FiredAt) ORDER BY (Action, FiredAt, ID, OriginID);
