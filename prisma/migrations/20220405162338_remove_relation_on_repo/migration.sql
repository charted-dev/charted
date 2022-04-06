/*
  Warnings:

  - You are about to drop the column `avatarHash` on the `organizations` table. All the data in the column will be lost.
  - You are about to drop the column `orgId` on the `repositories` table. All the data in the column will be lost.

*/
-- DropForeignKey
ALTER TABLE "repositories" DROP CONSTRAINT "repositories_orgId_fkey";

-- DropForeignKey
ALTER TABLE "repositories" DROP CONSTRAINT "repositories_ownerId_fkey";

-- AlterTable
ALTER TABLE "organizations" DROP COLUMN "avatarHash",
ADD COLUMN     "avatar_hash" TEXT;

-- AlterTable
ALTER TABLE "repositories" DROP COLUMN "orgId";
