CalvardTool 2.8: decompiles and builds the files of the Calvard games (PC versions)
  The Legend of Heroes: Trails through Daybreak     (Kuro no Kiseki)
  The Legend of Heroes: Trails through Daybreak II  (Kuro no Kiseki II)
  The Legend of Heroes: Trails beyond the Horizon   (Kai no Kiseki)

WHAT IT DOES
  Unpacks the game's .p3a archives into ordinary files for translation (dialogues .json, tables .xlsx, textures .png,
  interface text .yaml) and builds your edits back: into game archives or into one mod in the mods folder.
  Without edits, files build byte-for-byte identical to the original.
  Requires a graphics card with OpenGL 2 or later (any ordinary one; it may not start in virtual machines).

──────────────────────────────── QUICK START

  1. 'Game folder or files': 'Browse…' → the game folder or a translation mod's folder. You can also drop the game
     folder or .p3a files on the window.
  2. The 'What to decompile' window opens: tick what you need and click 'Done'.
  3. 'Project folder (for editing)' is where the files go; 'Output folder' is a separate folder for the result.
  4. 'Decompile all' → edit the files in the project folder.
  5. 'Build' → tick the folders in the 'What to build' window → 'Build'.
     Copy the result into the game folder, replacing the files. KEEP A COPY OF THE ORIGINALS
     (Steam may "repair" changed archives when it verifies the files).

──────────────────────────────── GAME

  Detected automatically: kuro.exe is Daybreak, kuro2.exe is Daybreak II, kai.exe or script_eng.p3a is Horizon.
  For a project or translation mod folder: by the folders inside (script_eng is Horizon, script_en is Daybreak / II)
  and by the folder name ("Daybreak_2_RU_V1.5.4", "Daybreak-2" is Daybreak II). The detected game is shown on the right
  above 'Game folder or files'; you can also choose it manually in the 'Game' list.
  Each game has its own project and output folders, and the program switches them when the game changes. On the first
  run the fields are empty: choose the folders yourself.
  A project folder remembers its game: the program won't decompile another game into it.
  The difference between the games: Daybreak and Daybreak II use script_en and table_en, Horizon uses script_eng and table_eng.

──────────────────────────────── DECOMPILING

  The 'What to decompile' window (opens by itself after you choose a folder; reopen it with the 'Files: N of M' button):
  • archives by section: Text for translation, Mods, Textures, Voices, Music and sounds, Models, Video,
    Other; each shows what's inside (scenes, tables, images), English or Japanese files, and the size.
    Japanese scenes and tables (script.p3a) are also decompiled to .json / .xlsx, into the project folder "script"
    (you can compare them with the translation in CalvardCompare and build them back);
  • everything is ticked by default; quick choices: 'Text only', 'Text+textures', 'All', 'Clear all';
    the tick on a section selects the whole section;
  • at the bottom: how much is selected, how much the archives weigh, how much the unpacked files will take,
    and roughly how long it will take (the time appears after the first decompile longer than 30 s
    on this computer; the program remembers its speed).
  Before decompiling: above 1 GB the program asks; if the disk lacks space it warns you.
  'File mask' decompiles only the files you need: c0100, t_item, c01* (comma-separated; empty means all).
  Files you have already edited are not overwritten when you decompile again.
  An installed mod (mods\*.p3a) takes priority over the game's archives: a file that exists in both the game and
  the mod goes only into the mod's folder. English originals of dialogues and tables replaced by the mod go
  into the project's _english folder, for CalvardCompare (they are not edited and don't go into the build).

  The project structure follows the game: each archive becomes a folder with the same name.
    Project\script_eng\script_eng\scena\a0000.json      ← from script_eng.p3a
    Project\script_eng\table_eng\t_name.xlsx
    Project\mods\RU_MOD\script_eng\scena\a0000.json     ← from mods\RU_MOD.p3a
    Project\multilanguage\message_en.yaml
  Don't touch the internal files (.kai_*.tsv, the _original and _english folders).

──────────────────────────────── WHAT BECOMES WHAT

  dialogues and scripts  .dat → .json  (Notepad++)
  tables                 .tbl → .xlsx  (Excel; tables without text are not exported)
  textures               .dds → .png   (or as they are, .dds, chosen in 'What to decompile'; rare formats are always .dds)
  interface text         multilanguage\message_en.yaml
  everything else (.mdl models, effects, sounds) stays as it is. To replace a model, put your file with the same name
  in place of the old one and build. Which model belongs to which character: character_models.txt.
  To get a model's files, use the CalvardTool extract command (see 'Command line').

  Dialogues (.json): translate only the strings in "text".
    \n is a line break in the window; {11}{1180} and other {numbers} are service codes, keep them in place;
    <C1>, <W500>, <#E_0#M_0#B_0> and other tags in <…> must not be touched; "code" and "ln" are internal fields.
    All text is extracted: lines, names, menus, answer choices.
  Tables (.xlsx): the 'Text' sheet: A is where the string is, B is the text (edit it), C is the original.
    Don't touch hidden column D or the '_internal' sheet, and don't delete or reorder rows.
    Fields with file names (the xipha model, chr0000_c15, an AI file and so on) are not exported to Excel; if such
    a field is changed anyway (for example with 'Replace all'), it stays as in the original when building,
    and the log shows the line "service fields … left as in the original".
  Textures (.png): save with transparency; better not to change the size.
  Textures (.dds): if you chose the .dds format when decompiling, the game's files are there as they are. Edit them in
    an editor with DDS support (Paint.NET, GIMP, a Photoshop plugin); only changed ones go into the build.

──────────────────────────────── BUILDING

  'Build mode':
  • 'Rebuild game archives': each project folder goes back into its own archive (script_eng → script_eng.p3a,
    mods\RU_MOD → mods\RU_MOD.p3a); only archives with changes are rebuilt.
    The output folder must differ from the game folder, because archives are built from the originals.
  • 'As one mod in mods': all changes go into one mods\<Mod name>.p3a; the game's archives are not touched.
  The 'What to build' window: the project folders by the same sections and how many files changed in each; at the bottom,
  roughly how much will be written and how long it will take (the time appears after the first build longer than 10 s).
  Only changed files of the ticked folders are built (in mod mode, unticked mod folders go into the mod
  in their original form). 'File mask' works here too.
  Reliability:
  • disk space is checked before archives are written; if it's short, the archives are not written;
  • an archive is written through a temporary file: if writing fails, the previous archive stays intact;
  • after writing, the archive is reopened, and every file is unpacked and compared with the built one
    ("verified: the archive reads…"); a damaged archive shows "✖" in the log and counts in the errors total;
  • before building into the game folder, the program asks for confirmation.

──────────────────────────────── THE DAYBREAK RUSSIAN TRANSLATION (Daybreak_RU)

  Install the mod in the game as usual (mods\RU_mod_….p3a and multilanguage) and decompile the whole game folder:
  English files go to Project\script_en\…, Russian ones to Project\mods\RU_mod_…\….
  In this translation, the text of many functions is written twice: an English copy first, then a Russian one
  ("fn … calls { … } { … }" in the .json). Translate the second, Russian one.
  Internal scripts without text that can't be read (for example mon9996_c00.dat) stay as they are.

──────────────────────────────── WINDOW

  • At the bottom are the 'Help' and 'Log' tabs (time, message; errors in red, 'Done' in green).
  • 'Open project', 'Open output' open the folders in Explorer; 'Stop' stops the work
    (when you close the window, the program waits until the current file is written).
  • A long decompile or build (over 10 s) ends with a Windows notification and a sound,
    and the program's taskbar button flashes.
  • Light and dark theme: the button at the bottom right.

──────────────────────────────── FILES NEXT TO THE PROGRAM

  CalvardTool.ini: settings (folders, mode, mod name, mask, decompile and build speed); CalvardTool.log: the log.
  character_models.txt: which model belongs to which character.

──────────────────────────────── COMMAND LINE

  CalvardTool decompile --game <game folder or .p3a file> --proj <project folder>
                    [--types scena,tables,textures,other] [--mask a0000,c01*] [--no-mods] [--overwrite]
  CalvardTool compile   --proj <project folder> --out <output folder> [--rebuild --game <game folder> | --name RU_MOD] [--changed]
  CalvardTool extract   --game <game folder> --proj <folder> --mask chr0000*
  CalvardTool file <file> [daybreak|daybreak2|horizon]: one .dat/.tbl/.dds to .json/.xlsx/.png and back
                    (.xlsx and .png need the original .tbl/.dds with the same name next to them; the result gets "_new")

Uses: Ingert (script decompiler for Kuro/Kai, Aureole Suite), KuroTools table schemas (MIT).
