# 0-shell

```
📁 simple-shell
├── 📁 src
│ ├── main.rs
│ ├── lib.rs
│ ├── 📁 commands # ⬅ Each command's logic
│ │ ├── mod.rs
│ │ ├── echo.rs
│ │ ├── cd.rs
│ │ ├── ...
│ ├── 📁 parser # ⬅ Input tokenizer and syntax parser
│ │ ├── mod.rs
│ │ └── parser.rs
│ ├── executor.rs # ⬅ Matches parsed command to a handler
│ └── utils.rs
├── Cargo.toml
└── README.md
```