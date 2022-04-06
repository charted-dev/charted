/*
  Warnings:

  - You are about to drop the column `icon_url` on the `repositories` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "repositories" DROP COLUMN "icon_url",
ADD COLUMN     "icon_hash" TEXT;
