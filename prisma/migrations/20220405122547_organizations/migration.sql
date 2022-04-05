/*
  Warnings:

  - You are about to drop the column `avatarUrl` on the `users` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "repositories" ADD COLUMN     "orgId" TEXT;

-- AlterTable
ALTER TABLE "users" DROP COLUMN "avatarUrl",
ADD COLUMN     "avatar_hash" TEXT;

-- CreateTable
CREATE TABLE "organizations" (
    "single_sasl_enabled" BOOLEAN NOT NULL DEFAULT false,
    "verified_publisher" BOOLEAN NOT NULL DEFAULT false,
    "twitterHandle" TEXT,
    "gravatarEmail" TEXT,
    "description" TEXT,
    "avatarHash" TEXT,
    "flags" INTEGER NOT NULL DEFAULT 0,
    "name" TEXT,
    "id" TEXT NOT NULL,

    CONSTRAINT "organizations_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "repositories" ADD CONSTRAINT "repositories_orgId_fkey" FOREIGN KEY ("orgId") REFERENCES "organizations"("id") ON DELETE SET NULL ON UPDATE CASCADE;
