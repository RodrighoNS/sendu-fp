[![GPLv3 License](https://img.shields.io/badge/License-GPL%20v3-yellow.svg)](https://opensource.org/licenses/)
![Crates.io Version](https://img.shields.io/crates/v/version)

# Sendu-fp

_Sendu Fast Printer_ allows to print labels from existing Work Orders from your registered Shop in [Sendu](https://app.sendu.cl/users/sign_in) using the default printer of the host OS.

## Features

- Direct printing using the OS default printer
- Enter ID manually or using a [Barcode reader](https://en.wikipedia.org/wiki/Barcode_reader)
- Plug-and-play
- Cross platform

## Installation

Before installation start, make sure your OS fulfill all the [prerequisites](https://v2.tauri.app/es/start/prerequisites/) needed.

After that, you can use:

```bash
  cargo tauri dev
```

To get the app running. Alternatively you can use the flag `--no-watch` in order to avoid refreshing the app every time you make a change.

```bash
  cargo tauri dev --no-watch
```

## Deployment

[WIP]

```bash
  # TODO: add github actions commands if needed
```

## Roadmap

- Auth capabilities to use any Sendu account.

- Selector to choose any print from the OS.

- A list with the latest labels printed.

- Improve UX.

## Authors

- [@RodrighoNS](https://github.com/RodrighoNS)

## License

[GNU AGPLv3](https://choosealicense.com/licenses/agpl-3.0/)
