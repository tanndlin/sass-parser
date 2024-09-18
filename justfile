# use PowerShell instead of sh:
set shell := ["powershell.exe", "-c"]

hello:
  Write-Host "Hello, world!"

# Build with cargo
build:
  cargo build -r

# Run the binary
run:
  cargo run -r

profile:
  flamegraph -o my_flamegraph.svg -- .\target\release\sass-parser.exe stress.scss

time:
  .\compare_time.ps1