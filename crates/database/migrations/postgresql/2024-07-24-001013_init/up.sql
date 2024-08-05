CREATE TABLE "repository_members"(
	"public_visibility" BOOL NOT NULL,
	"display_name" VARCHAR(32),
	"permissions" INT8 NOT NULL,
	"repository" INT8 NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL,
	"joined_at" TIMESTAMPTZ NOT NULL,
	"account" INT8 NOT NULL,
	"id" INT8 NOT NULL PRIMARY KEY,
	FOREIGN KEY ("repository") REFERENCES "repositories"("id")
);

CREATE TABLE "users"(
	"verified_publisher" BOOL NOT NULL,
	"gravatar_email" TEXT,
	"description" VARCHAR(240),
	"avatar_hash" TEXT,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL,
	"username" VARCHAR(64),
	"password" TEXT NOT NULL,
	"email" TEXT NOT NULL,
	"admin" BOOL NOT NULL,
	"name" VARCHAR(64) NOT NULL,
	"id" BIGINT NOT NULL PRIMARY KEY
);

CREATE TABLE "organizations"(
	"verified_publisher" BOOL NOT NULL,
	"twitter_handle" TEXT,
	"gravatar_email" TEXT,
	"display_name" VARCHAR(32),
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL,
	"icon_hash" TEXT,
	"private" BOOL NOT NULL,
	"owner" INT8 NOT NULL,
	"name" VARCHAR(32) NOT NULL,
	"id" INT8 NOT NULL PRIMARY KEY
);

CREATE TABLE "user_connections"(
	"github_account_id" TEXT,
	"google_account_id" TEXT,
	"apple_account_id" TEXT,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL,
	"user_id" BIGINT NOT NULL,
	"id" BIGINT NOT NULL PRIMARY KEY,
	FOREIGN KEY ("user_id") REFERENCES "users"("id")
);

CREATE TABLE "repositories"(
	"description" VARCHAR(140),
	"deprecated" BOOL NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL,
	"icon_hash" TEXT,
	"private" BOOL NOT NULL,
	"creator" BIGINT,
	"owner" BIGINT NOT NULL,
	"ty" CHARTTYPE NOT NULL,
	"name" VARCHAR(32) NOT NULL,
	"id" INT8 NOT NULL PRIMARY KEY
);

CREATE TABLE "api_keys"(
	"description" VARCHAR(140),
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL,
	"expires_in" TIMESTAMPTZ,
	"scopes" INT8 NOT NULL,
	"owner" INT8 NOT NULL,
	"token" TEXT NOT NULL,
	"name" VARCHAR(32) NOT NULL,
	"id" INT8 NOT NULL PRIMARY KEY,
	FOREIGN KEY ("owner") REFERENCES "users"("id")
);

CREATE TABLE "sessions"(
	"refresh_token" TEXT NOT NULL,
	"access_token" TEXT NOT NULL,
	"expires_in" TIMESTAMPTZ NOT NULL,
	"user" INT8 NOT NULL,
	"id" UUID NOT NULL PRIMARY KEY,
	FOREIGN KEY ("user") REFERENCES "users"("id")
);

CREATE TABLE "organization_members"(
	"public_visibility" BOOL NOT NULL,
	"display_name" VARCHAR(32),
	"organization" INT8 NOT NULL,
	"permissions" INT8 NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL,
	"joined_at" TIMESTAMPTZ NOT NULL,
	"account" INT8 NOT NULL,
	"id" INT8 NOT NULL PRIMARY KEY,
	FOREIGN KEY ("organization") REFERENCES "organizations"("id")
);

CREATE TABLE "repository_releases"(
	"repository" INT8 NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL,
	"tag" TEXT NOT NULL,
	"id" INT8 NOT NULL PRIMARY KEY,
	FOREIGN KEY ("repository") REFERENCES "repositories"("id")
);
