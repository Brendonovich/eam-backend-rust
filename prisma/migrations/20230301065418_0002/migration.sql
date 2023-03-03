/*
  Warnings:

  - A unique constraint covering the columns `[company_bid]` on the table `Company` will be added. If there are existing duplicate values, this will fail.
  - Added the required column `company_bid` to the `Company` table without a default value. This is not possible if the table is not empty.
  - Added the required column `telephone` to the `Company` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Company" ADD COLUMN     "company_bid" VARCHAR(255) NOT NULL,
ADD COLUMN     "telephone" VARCHAR(255) NOT NULL;

-- CreateIndex
CREATE UNIQUE INDEX "Company_company_bid_key" ON "Company"("company_bid");
