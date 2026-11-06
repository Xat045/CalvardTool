CalvardCompare 0.2: translation compare for Trails through Daybreak, Daybreak II, Trails beyond the Horizon

WHAT IT DOES
  Shows the text of two languages side by side, line by line (Japanese, English or Russian, in any pair,
  usually the original on the left and the translation on the right), with search across both columns and a dictionary.
  The 'Checks' tab shows translation errors: lost service codes, untranslated lines, placeholders.
  CalvardCompare only reads translation files and writes nothing to them.
  Requires a graphics card with OpenGL 2 or later.

──────────────────────────────── QUICK START

  1. The left field, 'English/Japanese/Russian files', is the original, one of:
     • the game folder itself (the one with script_eng.p3a or script_en.p3a): CalvardCompare unpacks the English
       dialogues and tables from the archive itself, up to a minute the first time, then from the cache (about a second);
     • a CalvardTool project folder with English files;
     • Japanese files: CalvardTool decompiles the game's Japanese scenes and tables (the script.p3a archive) into the
       project folder "script"; choose that folder (Project\script). If a translation was made from Japanese,
       this way 'Checks' compares codes with the text it was translated from.
  2. The right field is the translation: the CalvardTool project folder where the translation is decompiled (mods\RU_MOD\…).
     You can choose the same project folder in both fields: Russian comes from mods, English from the
     game files and the _english folder.
     Folders can be dropped on the window: the left half sets the left field, the right half the right one
     (two folders at once: the first on the left, the second on the right).
  3. 'Load'. Type a word and press Enter.

──────────────────────────────── LOADING

  • On start, CalvardCompare loads the last folders and opens where you stopped:
    the same search word and line, the same tab, 'Checks' section, expanded group and scroll position.
  • The load cache is the CalvardCompare_cache folder next to the program: unchanged files are not read again.
    Shift + 'Load' (or Shift+F5) reloads everything without the cache.
  • Above the button is how much has loaded; if a file was not read, a red label appears there (hover over it).
  • Files from different games: if the English files are from one game and the Russian from another, lines are not
    shown and a warning appears instead. The game is detected from the folder (exe, archives, script_en / script_eng
    folders, the folder name) and, failing that, by how well script commands match. If it's wrong, use 'Show anyway'.
  • Lines are matched by script structure, not by number: an extra line doesn't shift the others.
    Large scenes are matched first by script functions, then by commands and the line's leading tag (<k>, <KR>…).
    '- none -' means this language has no such line. '-DO NOT TOUCH-' means there is text, but it's a service name
    (TXT_SAVE_NIGHTMARE, chr0000_c15), and editing it breaks the game; the name itself is next to it in gray.
  • In the Daybreak translation, where a function's text is written twice ("calls { … } { … }"), the Russian copy is shown.

──────────────────────────────── SEARCH AND THE TABLE

  • Lines where the word is in at least one of the two columns are shown.
    The Cyrillic letters yo and ye count as one; dashes of any kind, quotes and ellipses count as the same. 'Match case':
    "Van" won't find "van". Several words find lines that have all of them. 'Whole words': mare finds "Mare's"
    but not "Nightmare"; without the tick, part of a word is found too.
  • 'Whole file' shows all lines of one file in order.
  • Blue is a search match, yellow is a dictionary word and its translation.
  • Text wraps; column widths can be changed by dragging the border.
    Text size: Ctrl+wheel or Ctrl+plus / minus, Ctrl+0 for normal (remembered).
  • The 'File' column shows the file name and the line number in it.
  • Right-click a line: 'Copy the left column / right column / both / the file name',
    'Open the left file', 'Open the right file': .json, .yaml and .txt open in Notepad++ at this line
    (without Notepad++, in the default program); an .xlsx table opens in Excel or WPS on the 'Text' sheet
    with cell B of the line selected (if the table is already open, in that window).
  • Ctrl+C copies the clicked column. Double-clicking a cell opens a field with the full text
    so you can select and copy part of it. F11 is full screen.
  • Scrolling with the pressed wheel: press the wheel and move the mouse and the table scrolls by itself; press again to stop.
  • Hotkeys are at the end of this file.

──────────────────────────────── DICTIONARY

  Choose a dictionary with the 'Choose' button (.dat, .json or .txt).
  The file can be updated without closing the program: it is reread on the next search.
  Formats:
    JSON: {"Bracer": "translation", "Spriggan": ["variant 1", "variant 2"]} or [{"en": …, "ru": …}],
          comments //, #, /* */, "_comment"; an extra comma is harmless;
    text (UTF-8 or Windows-1251): Bracer = translation  /  Bracer:Translation  /  Shizuna:form1-form2-form3
          (case forms with dashes if they start the same; variants with commas; # is a comment).
  • The search word is highlighted if it's in the dictionary, in the column of the language you typed it in;
    first and last names separately; in Russian, with word endings.
  • Next to 'Found' is how many times the Russian variant occurs in the found lines.
  • 'Mark as in the dictionary': a line is green if, for each dictionary word in the English text,
    the Russian has its dictionary translation. With 'Whole words', only exact forms from the dictionary count.
    A phrase of several words is checked only when the whole phrase is in the English.

──────────────────────────────── THE CHECKS TAB

  Run in the background right after loading; the tab shows the number of errors.
  • 'Service codes': lost or extra codes (<C…>/</C>, {11}{…}, {16}, <K>, <E…>, <B>…),
    a broken line break ('\' without n, or '\\n' as text), a changed service name ('-DO NOT TOUCH-'),
    a .json file that can't be read (with the line number). What each code means: the 'Service codes' section at the end of this file.
  • 'Not translated': scene lines left in English or Japanese (tables and the interface
    are not checked); identical ones on one line, with the count in brackets. Debug scenes (a0…, debug,
    sample) are not checked. If the left side is Japanese, its typography (furigana <R>…, size <s24>,
    bold <B>, color <c…>…</C>) is not compared with the translation.
  • 'Placeholders': %d, %s, %.1f… are places where the game inserts a number or a name. If the translation
    doesn't have as many as the original, the game shows the wrong thing or crashes.
  • Identical errors are grouped: 'missing: </C>, 120 lines', the files and an example.
    'Show all' (or a click on the group) expands the group's lines; 'Not an error · all' hides the group.
  • The 'dialogues .json', 'tables .xlsx', 'interface .yaml' buttons show or hide findings of that type.
  • 'Open in Compare' (or a double click) goes to the line on the 'Compare' tab. Right-click a finding for
    a menu: 'Open in Compare', 'Open the left file', 'Open the right file' (at this line).
  • 'Not an error' removes a finding; 'Hidden' shows everything removed, with 'Restore' / 'Restore all'.
  • 'Export to Excel': findings to .xlsx, one sheet per section; each finding has 5 lines before and after
    from the same file, as they are, with service codes (the finding in yellow), so you can see what's missing.
  • 'Check again' after edits (click 'Load' first).
  • Scrolling with the pressed wheel: press the wheel and move the mouse and the list scrolls by itself; press again to stop.

──────────────────────────────── PORTRAITS

  Lines from t_active_voice.xlsx (characters' on-the-move phrases) show the speaker's portrait on the left in the file column.
  The speaker comes from the table record (character number → their model in t_name), and the image is the same as in
  the game next to the phrase (from face_c####, or btlface0_c#### for those without one). The facial expression is ignored.
  Unnamed NPCs have no portraits. Portraits are in "data.dat" (each game has its own).
  Scene lines get a portrait by the speaker's ID (system[5,6](ID, ...) to their model). These are the same portraits
  as for t_active_voice: a character without one stays without a portrait.

──────────────────────────────── ICONS INSTEAD OF <I…> CODES

  On screen only; nothing is written to files. The 'Icons: …' button in the settings row opens the window.
  • Images are in "data.dat" next to the program (encrypted; unpack the whole archive).
    Choose a game in the window: Daybreak 1, Daybreak 2 or Horizon (Horizon by default). Each has its own
    icons.png (items, statuses, '!' dots: codes <I0>…<I440>); the ph3_button_prompts.png buttons are shared:
    PlayStation, Xbox, Switch, Steam Deck, Stadia, keyboard and mouse. 'Device' sets whose icons are shown.
  • Codes 900 and up are "actions" (menu, attack, shard search). Most are assigned from the game's layout files
    and screens and show right away (for example 900 ×, 901 ○, 912 R3, 1016 L1, 1120–1129: the
    L2 quick access menu). A code without an icon is a gray frame with a number; assign it once in the window:
    choose the code and click a button in the palette (Ctrl+click for a combination, for example 910 = L2+R2).
    A gamepad button is assigned by position, for all gamepads at once; the keyboard separately.
  • Icons in tables too, as in the game: t_item items (from t_item.tbl in the project's _original), t_skill crafts
    (type and power), t_condition_info statuses and t_itemhelp hints, craft ranges, effects, elements,
    and interface text where the game draws an icon itself (map legend, rewards, menu buttons).

──────────────────────────────── FILES NEXT TO THE PROGRAM

  KaiCompare.ini: settings (folders, dictionary, theme, where you stopped, text size);
  CalvardCompare_buttons.txt: button assignments;
  CalvardCompare_cache: the cache of loading and of English files from the game folder, plus CalvardCompare_hidden.txt,
  the 'Not an error' list (Shift+'Load' doesn't delete it);
  data.dat: icons and portraits.

COMMAND LINE
  CalvardCompare <English folder> <Russian folder> <word>: search in the console (DICT=<dictionary> adds highlights).

──────────────────────────────── HOTKEYS

Ctrl+F          go to the search field (from any tab)
Enter           search (in the search field)
F5              reload the folders
Shift+F5        reload without the cache (same as Shift + the 'Load' button)
Ctrl+1          the 'Compare' tab
Ctrl+2          the 'Checks' tab
Ctrl+C          copy the selected line's clicked column
Ctrl+wheel      larger / smaller text in the 'Compare' table (remembered)
Ctrl+plus/minus the same with keys; Ctrl+0 for normal size
F11             full screen; Esc leaves full screen
Esc             close a window or menu

Mouse
Double-click a finding in 'Checks'         open this line in 'Compare'
Right-click a line in 'Compare'            menu (copy, open the left / right file…)
Right-click a finding in 'Checks'          menu (open in Compare, open the left / right file)
Press the wheel in a table or list         the list scrolls where you move the mouse; press again to stop
Drag a folder onto the window              left half: the left field, right half: the right one

On the next start, CalvardCompare opens where you stopped:
the same search word and line, the same tab, the same 'Checks' section and expanded group.

──────────────────────────────── SERVICE CODES

The main rule: every code in the English line must remain in the translated one, the same codes and as many of them.
Only the text between them is translated. CalvardCompare compares the codes in each line and shows
what's missing or extra.

The notes below come from the game's own debug scenes (where the developers labeled what a code does)
and from how the codes sit in the text. Where a purpose isn't confirmed, it says so.
In brackets: how many times the code occurs in the English text of the three games.

──────────────────────────────── LINE BREAK

\n      a new line in the dialogue window. In .json it is written exactly so: a backslash and the letter n.
        If you write only '\' (without n), the file won't be read or the dialogue won't continue.
        If you write '\\n', the game shows '\n' as text. CalvardCompare catches both cases.

──────────────────────────────── THE SPEAKER'S FACE (portrait window)

<K>     the speaker's portrait, normal position, presumably (11 336)
<KL>    portrait on the left, presumably, by analogy with <KR> (7 265)
<KR>    portrait on the right (14 449); in the game's debug: "Face right"
<k>     portrait on the left, smaller (13 732); in debug: "Face left - smaller"
<kR>    portrait on the right, larger (14); in debug: "Face right - larger"
        They go at the start of a line. Don't delete them or move them to the middle.

──────────────────────────────── TEXT COLOR AND STYLE

<C1> … <C9>   start of colored text (the number is a color from the game's palette) (11 442)
</C>          end of colored text, back to the normal color (812)
              If </C> is lost, everything after that point takes the <C…> color.
              <C> without a number is rare, apparently also a color reset (not confirmed).
<c698> … </c> a color by RGB code; in debug: "Change text color (RGB)" (78 / 21)
<B> … </B>    bold text; in debug: "Set whole row of text to bold" (72 / 75)
<S3>          text size; in debug: "Change text size" (1 392)
<s40>         text size (small variant), for example "<s40>The next day..." (183)
{tab}         indent (tab) (34, Horizon only)

──────────────────────────────── SPEED AND PAUSES

<W200>   text display speed; in debug: "Change text display speed" (784)
<w1000>  pause in text display (the number is milliseconds); in debug:
         "Stop text display for a set amount of seconds" (454)

──────────────────────────────── EMOTIONS AND ICONS

<E0>     an emotion above a character; in debug: "Display PA emotion" (323)
<Es5>    an emotion without sound; in debug: "Display emotion (No SE)" (6)
<#E_1>, <#E_A>, <#S[1]>, <#P_1>, <#H[2]>  special effects in a line (isolated cases,
         purpose not confirmed); leave them as in the original
<I210>   a button or icon in the text (for example the O button); in debug: "O Button" (164)
<R>      occurs once in Horizon's debug; purpose unknown

──────────────────────────────── CODES IN CURLY BRACKETS {number}

{number}  the game's service values (3 212). They come in pairs or alone, usually at the end of a line
          or in place of a name. What is known:
  {21}{2007}   a craft's name (S-Craft) by number; the game inserts the name itself
  {17}{2005}   an item's name by number
  {11}{41957}  a link to the line's voice (voice file number). If it's lost,
               the line may lose its voice
  {16}, {13}, {7}  go at the end of a line; they look like waiting for a key press / closing the window,
               exact purpose not confirmed
  {0} and others: purpose unknown
          Keep all {numbers} exactly as in the original, in the same place.

──────────────────────────────── PLACEHOLDERS

$1, $2   in the "code" field: places where the game inserts text. The "text" strings don't touch them.
