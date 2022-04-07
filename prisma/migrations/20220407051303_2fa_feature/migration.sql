-- CreateTable
CREATE TABLE "2fa_google" (
    "owner_id" TEXT NOT NULL,
    "otp_code" TEXT NOT NULL,
    "id" TEXT NOT NULL,

    CONSTRAINT "2fa_google_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "2fa_other" (
    "owner_id" TEXT NOT NULL,
    "otp_code" TEXT NOT NULL,
    "id" TEXT NOT NULL,

    CONSTRAINT "2fa_other_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "2fa_google_owner_id_key" ON "2fa_google"("owner_id");

-- CreateIndex
CREATE UNIQUE INDEX "2fa_other_owner_id_key" ON "2fa_other"("owner_id");

-- AddForeignKey
ALTER TABLE "2fa_google" ADD CONSTRAINT "2fa_google_owner_id_fkey" FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "2fa_other" ADD CONSTRAINT "2fa_other_owner_id_fkey" FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
