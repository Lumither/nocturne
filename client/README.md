## Module Structure

```shell
.
├── README.md
├── cli
├── content
│   ├── pages
│   │   └── about
│   │       └── <static_page>.md
│   └── posts
│       └── <year>
│           └── <identifier>
│               ├── assets
│               │   └── <assets>
│               ├── translate_<lang>.md
│               └── index.md
├── requirements.txt
├── templates
│   ├── <template>.py
│   └── ...
└── utils
    └── ...
```

### About `posts`

- Will render to `https://<DOMAIN>/blog/post/<identifier>`.
- Files under `assets/` (where images etc. located) should be pushed to OSS.
- For identifiers, use prefix `-` to indicate private (hidden) visibility. EX: `-hello-world`.