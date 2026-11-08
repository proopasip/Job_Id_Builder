@echo off
echo [1/3] Installing dependencies...
call npm install

echo [2/3] Cleaning previous builds...
if exist dist rd /s /q dist
if exist out rd /s /q out

echo [3/3] Building application...
call npm run build

echo.
echo Build completed successfully!
pause
