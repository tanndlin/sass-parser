just build

Measure-Command { sass stress.scss > stress.css}
Measure-Command { target/release/sass-parser.exe stress.scss}
