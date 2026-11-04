# CalvardTool

Building, extraction and proofreading of texts for The Legend of Heroes games (PC, Steam): Trails through Daybreak, Trails through Daybreak II and Trails beyond the Horizon.

| Program | What it does |
|---|---|
| **CalvardTool** (`CalvardTool.exe`) | Unpacks the game's `.p3a` archives into editable files (dialogues to `.json`, tables to `.xlsx`, textures to `.png` or `.dds`, interface text to `.yaml`) and builds your edits back into game archives or a single mod in the `mods` folder. Without edits, files build byte-for-byte identical to the originals. |
| **CalvardCompare** (`CalvardCompare.exe`) | Shows the text of two languages side by side, line by line, with search and a dictionary. The Checks tab finds translation errors: lost service codes, untranslated lines, broken placeholders. It only reads translation files and never writes to them. |

The interface is in English and Russian. It starts in English; the **RU / EN** button at the bottom of CalvardTool and at the top of CalvardCompare switches the language, and the choice is remembered. CalvardCompare's checks and dictionary are built around a Russian translation: they treat Cyrillic text as translated and compare English against Russian.

## Screenshots

CalvardTool:

![CalvardTool main window](docs/screenshot_tool.jpg)

![Choosing what to decompile](docs/screenshot_tool_files.jpg)

CalvardCompare:

![Comparing two languages line by line](docs/screenshot_compare.jpg)

![Translation checks](docs/screenshot_checks.jpg)

## How to use

1. Download the archives from [Releases](../../releases) and unpack them anywhere. `data.dat` must stay next to `CalvardCompare.exe`: it holds the game icons and portraits shown in the text.
2. In CalvardTool, choose the game folder, tick what to decompile, then click **Decompile all**.
3. Edit the files in the project folder.
4. Click **Build** and copy the result into the game folder, or use the mod it creates. Keep a copy of the originals: Steam may restore changed archives when it verifies the files.

Full guides: [CalvardTool](docs/ReadMe_CalvardTool.txt), [CalvardCompare](docs/ReadMe_CalvardCompare.txt). In Russian: [CalvardTool](docs/ReadMe_CalvardTool_RU.txt), [CalvardCompare](docs/ReadMe_CalvardCompare_RU.txt). Which model belongs to which character: [character_models.txt](docs/character_models.txt).

## Building from source

You need Windows, [Rust](https://rustup.rs) and Visual Studio Build Tools with a Windows SDK. The toolchain version is set in `rust-toolchain.toml`.

```
cargo build --release -p calvard-tool
```

The programs appear in `target\release`. CalvardCompare's `data.dat` is not in the repository because it holds images taken from the games; it is included in the release archives.

## Credits

This project includes modified copies of these libraries in the `libs` folder. All credit for them goes to their authors:

- **Kyuuhachi**, [Aureole Suite](https://aureole.kyuuhachi.dev): `calmare`, `themelios` and its `themelios-*` crates, `cradle`, `gospel`, `gospel-dump`, `cp932`, `bc7`, `bzip`.
- **Kyuuhachi**, Ingert, the script decompiler for Trails through Daybreak and later games: `ingert`, `ingert-syntax`.
- [KuroTools](https://github.com/nnguyen259/KuroTools) (MIT): table schemas.
- [Lucide](https://lucide.dev): interface icons, see [the license](tool/src/icons/LICENSE).

The Legend of Heroes and all game assets belong to Nihon Falcom. This is an unofficial fan project, and you need your own copy of the games.
