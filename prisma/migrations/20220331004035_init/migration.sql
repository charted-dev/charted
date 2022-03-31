-- CreateTable
CREATE TABLE "users" (
    "gravatar_email" TEXT,
    "description" TEXT,
    "avatarUrl" TEXT,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "username" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "email" TEXT NOT NULL,
    "flags" INTEGER NOT NULL DEFAULT 0,
    "name" TEXT,
    "id" TEXT NOT NULL,

    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "repositories" (
    "stargazers_count" INTEGER NOT NULL DEFAULT 0,
    "downloads_count" INTEGER NOT NULL DEFAULT 0,
    "description" TEXT,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "icon_url" TEXT,
    "ownerId" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "id" TEXT NOT NULL,

    CONSTRAINT "repositories_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "user_connections" (
    "noelware_account_id" TEXT,
    "google_account_id" TEXT,
    "apple_account_id" TEXT,
    "ownerId" TEXT NOT NULL,
    "id" TEXT NOT NULL,

    CONSTRAINT "user_connections_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "users_username_key" ON "users"("username");

-- CreateIndex
CREATE UNIQUE INDEX "users_email_key" ON "users"("email");

-- CreateIndex
CREATE UNIQUE INDEX "user_connections_noelware_account_id_key" ON "user_connections"("noelware_account_id");

-- CreateIndex
CREATE UNIQUE INDEX "user_connections_google_account_id_key" ON "user_connections"("google_account_id");

-- CreateIndex
CREATE UNIQUE INDEX "user_connections_apple_account_id_key" ON "user_connections"("apple_account_id");

-- CreateIndex
CREATE UNIQUE INDEX "user_connections_ownerId_key" ON "user_connections"("ownerId");

-- AddForeignKey
ALTER TABLE "repositories" ADD CONSTRAINT "repositories_ownerId_fkey" FOREIGN KEY ("ownerId") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "user_connections" ADD CONSTRAINT "user_connections_ownerId_fkey" FOREIGN KEY ("ownerId") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
