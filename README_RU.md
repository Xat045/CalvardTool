# CalvardTool

[English](README.md) · **Русский**

Сборка, извлечение и вычитка текстов игр The Legend of Heroes (PC, Steam): Trails through Daybreak, Trails through Daybreak II и Trails beyond the Horizon.

| Программа | Что делает |
|---|---|
| **CalvardTool** (`CalvardTool.exe`) | Разбирает архивы игры `.p3a` на файлы для правки (диалоги в `.json`, таблицы в `.xlsx`, текстуры в `.png` или `.dds`, надписи интерфейса в `.yaml`) и собирает правки обратно: в архивы игры или в один мод в папке `mods`. Без правок файлы собираются байт в байт как оригинал. |
| **CalvardCompare** (`CalvardCompare.exe`) | Показывает текст двух языков рядом, строка к строке, с поиском и словарём. На вкладке «Проверки» находит ошибки перевода: потерянные служебные коды, непереведённые строки, сломанные подстановки. Файлы перевода программа только читает и ничего в них не пишет. |

Интерфейс на английском и русском. При первом запуске он английский; кнопка **RU / EN** переключает язык и запоминает выбор. В CalvardTool она внизу окна, в CalvardCompare вверху. Проверки и словарь CalvardCompare рассчитаны на русский перевод: текст на кириллице считается переведённым, а английский сравнивается с русским.

## Скриншоты

CalvardTool:

![Главное окно CalvardTool](docs/screenshot_tool.jpg)

![Выбор файлов для разборки](docs/screenshot_tool_files.jpg)

CalvardCompare:

![Сравнение двух языков по строкам](docs/screenshot_compare.jpg)

![Проверки перевода](docs/screenshot_checks.jpg)

## Как пользоваться

1. Скачайте архивы из [Releases](../../releases) и распакуйте в любую папку. `data.dat` должен лежать рядом с `CalvardCompare.exe`: в нём значки и портреты из игр, которые показываются в тексте.
2. В CalvardTool укажите папку игры, отметьте, что разбирать, и нажмите **Декомпилировать всё**.
3. Правьте файлы в папке проекта.
4. Нажмите **Собрать** и скопируйте результат в папку игры или используйте собранный мод. Сохраните копию оригиналов: Steam может восстановить изменённые архивы при проверке файлов.

Подробные инструкции: [CalvardTool](docs/ReadMe_CalvardTool_RU.txt), [CalvardCompare](docs/ReadMe_CalvardCompare_RU.txt). Какая модель у какого персонажа: [character_models.txt](docs/character_models.txt) (на английском).

## Сборка из исходников

Нужны Windows, [Rust](https://rustup.rs) и Visual Studio Build Tools с Windows SDK. Версия компилятора указана в `rust-toolchain.toml`.

```
cargo build --release -p calvard-tool
```

Программы появятся в `target\release`. `data.dat` для CalvardCompare в репозитории нет, потому что в нём картинки из игр; он лежит в архивах релиза.

## Авторы

В папке `libs` лежат изменённые копии этих библиотек. Вся заслуга за них принадлежит их авторам:

- **Kyuuhachi**, [Aureole Suite](https://aureole.kyuuhachi.dev): `calmare`, `themelios` и его крейты `themelios-*`, `cradle`, `gospel`, `gospel-dump`, `cp932`, `bc7`, `bzip`.
- **Kyuuhachi**, Ingert, декомпилятор скриптов Trails through Daybreak и более поздних игр: `ingert`, `ingert-syntax`.
- [KuroTools](https://github.com/nnguyen259/KuroTools) (MIT): схемы таблиц.
- [Lucide](https://lucide.dev): значки интерфейса, см. [лицензию](tool/src/icons/LICENSE).

The Legend of Heroes и все материалы игр принадлежат Nihon Falcom. Это неофициальный фанатский проект, для работы нужны ваши собственные копии игр.
