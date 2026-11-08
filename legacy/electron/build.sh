#!/bin/bash
echo "[1/3] Installing dependencies..."
npm install

echo "[2/3] Cleaning previous builds..."
rm -rf dist out

echo "[3/3] Building application..."
npm run build

echo ""
echo "Build completed successfully!"
