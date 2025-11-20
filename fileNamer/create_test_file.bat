@echo off
setlocal

REM 현재 bat 파일이 위치한 폴더 기준 test 폴더 경로
set TARGET=%~dp0test

echo Target folder: %TARGET%

REM test 폴더 없으면 생성
if not exist "%TARGET%" (
    mkdir "%TARGET%"
)

REM test 폴더 초기화(파일 삭제)
echo Cleaning old test files...
del /q "%TARGET%\*" >nul 2>&1

echo Creating test files...

type nul > "%TARGET%\IMG_001.JPG"
type nul > "%TARGET%\IMG 002.JPG"
type nul > "%TARGET%\my document.txt"
type nul > "%TARGET%\example File.PDF"
type nul > "%TARGET%\sample_image.png"
type nul > "%TARGET%\untitled.TXT"
type nul > "%TARGET%\hello world.jpg"
type nul > "%TARGET%\TestFile.Mp4"
type nul > "%TARGET%\DATA_01.csv"
type nul > "%TARGET%\data_01 (copy).csv"

echo Done!
pause
