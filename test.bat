@echo off
cd /d %~dp0

ping /n 1 mozilla.org | nkf -w -Lu -d
