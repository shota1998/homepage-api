#!/usr/bin/env bash

touch .env

echo DATABASE_URL       = ${{ secrets.DATABASE_URL }}       >> .env
echo DATABASE_URL_TEST  = ${{ secrets.DATABASE_URL }}       >> .env

echo ALLOWED_ORIGIN_1   = ${{ secrets.ALLOWED_ORIGIN_1 }}   >> .env
echo ALLOWED_ORIGIN_2   = ${{ secrets.ALLOWED_ORIGIN_2 }}   >> .env

echo LOCAL_FILE_STORAGE = ${{ secrets.LOCAL_FILE_STORAGE }} >> .env

echo AWS_REGION         = ${{ secrets.AWS_REGION }}         >> .env

echo AWS_BUCKET_TEST    = ${{ secrets.AWS_BUCKET_TEST }}    >> .env
echo AWS_BUCKET_DEV     = ${{ secrets.AWS_BUCKET_DEV }}     >> .env
echo AWS_BUCKET         = ${{ secrets.AWS_BUCKET }}         >> .env

echo AWS_KEY_ID         = ${{ secrets.AWS_KEY_ID }}         >> .env
echo AWS_KEY_SECRET     = ${{ secrets.AWS_KEY_SECRET }}     >> .env

echo FILE_STORAGE_LOCATION = ${{ secrets.FILE_STORAGE_LOCATION }} >> .env

cat .env