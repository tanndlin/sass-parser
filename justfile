# use PowerShell instead of sh:
set shell := ["powershell.exe", "-c"]

hello:
  Write-Host "Hello, world!"

# Build with cargo
build:
  cargo build -r