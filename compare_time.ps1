just build

Measure-Command { sass stress.scss > stress.css }
target/release/sass-parser.exe stress.scss
