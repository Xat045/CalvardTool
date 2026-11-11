#[allow(unused_imports)]
use calvard_tool::{leprintln, lformat, lprintln};
#[allow(unused_imports)]
use calvard_tool::lang::tr;
use eframe::egui::{self, Align2, Color32, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::gui_common::*;
use crate::kai_pick::{Event as PEvent, Item as PItem, Mode as PMode, Picker};
use calvard_tool::jobs;
use calvard_tool::kai::game::Game;
use calvard_tool::kai::jobs::{self as kjobs, Options};

const TITLE: &str = concat!("CalvardTool ", env!("VER_CALVARDTOOL"), env!("VER_SUFFIX"), " — Trails through Daybreak / Daybreak II / Trails beyond the Horizon");
fn game_items() -> [&'static str; 4] {
	[tr("Определить по папке игры", "Auto-detect"), "Trails through Daybreak", "Trails through Daybreak II", "Trails beyond the Horizon"]
}

#[derive(Default)]
struct Shared {
	log: Mutex<Vec<String>>,
	done: AtomicUsize,
	total: AtomicUsize,
	finished: AtomicBool,
	cancel: AtomicBool,
	pause: AtomicBool,
	current: Mutex<String>,
	check: Mutex<Vec<String>>,
}

struct GuiSink {
	shared: Arc<Shared>,
	ctx: egui::Context,
}

fn log_path() -> PathBuf {
	exe_dir().join("CalvardTool.log")
}

impl jobs::Sink for GuiSink {
	fn log(&self, s: &str) {
		self.shared.log.lock().unwrap().push(s.to_string());
		use std::io::Write;
		if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(log_path()) {
			let _ = f.write_all(format!("{s}\r\n").as_bytes());
		}
		self.ctx.request_repaint();
	}
	fn progress(&self, done: usize, total: usize) {
		self.shared.done.store(done, Ordering::SeqCst);
		self.shared.total.store(total, Ordering::SeqCst);
		self.ctx.request_repaint();
	}
	fn cancelled(&self) -> bool {
		while self.shared.pause.load(Ordering::SeqCst) && !self.shared.cancel.load(Ordering::SeqCst) {
			std::thread::sleep(std::time::Duration::from_millis(100));
		}
		self.shared.cancel.load(Ordering::SeqCst)
	}
	fn current(&self, name: &str) {
		*self.shared.current.lock().unwrap() = name.to_string();
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Op {
	Decompile,
	Compile,
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
	Start,
	Normal,
	Done,
	Warn,
}

struct LogLine {
	time: String,
	text: String,
	kind: Kind,
}

#[derive(Clone)]
enum Action {
	Start(Op),
	ForceBuild,
}

fn sel_game(sel: usize) -> Option<Game> {
	match sel {
		1 => Some(Game::Daybreak),
		2 => Some(Game::Daybreak2),
		3 => Some(Game::Horizon),
		_ => None,
	}
}


fn now() -> String {
	let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) as i64 + local_offset();
	let s = t.rem_euclid(86400);
	format!("{:02}:{:02}:{:02}", s / 3600, (s / 60) % 60, s % 60)
}

fn local_offset() -> i64 {
	#[cfg(windows)]
	{
		#[repr(C)]
		struct SystemTime {
			y: u16, m: u16, dow: u16, d: u16, h: u16, mi: u16, s: u16, ms: u16,
		}
		#[repr(C)]
		struct Tzi {
			bias: i32,
			sname: [u16; 32],
			sdate: SystemTime,
			sbias: i32,
			dname: [u16; 32],
			ddate: SystemTime,
			dbias: i32,
		}
		unsafe extern "system" {
			fn GetTimeZoneInformation(tz: *mut Tzi) -> u32;
		}
		unsafe {
			let mut tz: Tzi = std::mem::zeroed();
			let r = GetTimeZoneInformation(&mut tz);
			let extra = if r == 2 { tz.dbias } else { tz.sbias };
			-((tz.bias + extra) as i64) * 60
		}
	}
	#[cfg(not(windows))]
	{
		0
	}
}

struct App {
	pal: Pal,
	settings: Settings,
	src: String,
	proj: String,
	out: String,
	mod_name: String,
	mask: String,
	game_sel: usize,
	mode: usize,
	tab: usize,
	page: usize,
	log: Vec<LogLine>,
	log_to_end: bool,
	shared: Option<Arc<Shared>>,
	op: Option<Op>,
	op_start: Instant,
	last_done: Option<Op>,
	status: String,
	flash: Option<(String, Instant)>,
	dialog: Dialog<Action>,
	first_frame: bool,
	pick: Option<Picker>,
	dec_sel: Option<(String, Vec<PItem>)>,
	build_est: u64,
	build_once: Option<Vec<String>>,
	other_files: bool,
	keep_failed: bool,
	tex_dds: bool,
	tables_json: bool,
	skip_check: bool,
	last_build: Option<Vec<String>>,
	build_ready: bool,
	det_cache: std::cell::RefCell<Option<(String, Option<Game>)>>,
}

impl App {
	fn new(cc: &eframe::CreationContext) -> App {
		setup_fonts(&cc.egui_ctx);
		set_dialog_owner(cc);
		egui_extras::install_image_loaders(&cc.egui_ctx);
		if !exe_dir().join("CalvardTool.ini").exists() && exe_dir().join("KaiTool.ini").exists() {
			let _ = std::fs::copy(exe_dir().join("KaiTool.ini"), exe_dir().join("CalvardTool.ini"));
		}
		let st = Settings::load("CalvardTool.ini", "CalvardTool");
		UI_EN.store(st.get("lang") != Some("ru"), std::sync::atomic::Ordering::Relaxed);
		let dark = st.get("dark") == Some("1");
		let pal = Pal::get(dark);
		apply_visuals(&cc.egui_ctx, &pal);
		let game_sel = st.get("title").and_then(|v| v.parse::<usize>().ok()).filter(|&v| v < 4).unwrap_or(0);
		let g = sel_game(game_sel);
		let key_p = g.map(|g| format!("proj_{}", g.key())).unwrap_or_else(|| "proj".into());
		let key_o = g.map(|g| format!("out_{}", g.key())).unwrap_or_else(|| "out".into());
		let proj = st.get(&key_p).or(st.get("proj")).map(|s| s.to_string()).unwrap_or_default();
		let out = st.get(&key_o).or(st.get("out")).map(|s| s.to_string()).unwrap_or_default();
		crate::kai_pick::set_speed(st.get("unpack_speed").and_then(|v| v.parse().ok()).unwrap_or(0.0));
		crate::kai_pick::set_build_speed(st.get("build_speed").and_then(|v| v.parse().ok()).unwrap_or(0.0));
		
		App {
			pal,
			src: st.get_or("game", "").to_string(),
			proj,
			out,
			mod_name: st.get_or("mod_name", "RU_MOD").to_string(),
			mask: st.get_or("mask", "").to_string(),
			game_sel,
			mode: if st.get("mode") == Some("mod") { 1 } else { 0 },
			tab: 0,
			page: if st.get("page") == Some("1") { 1 } else { 0 },
			log: Vec::new(),
			log_to_end: false,
			shared: None,
			op: None,
			op_start: Instant::now(),
			last_done: None,
			status: tr("Готов к работе", "Ready").into(),
			flash: None,
			dialog: Dialog::None,
			first_frame: true,
			pick: None,
			dec_sel: None,
			build_est: 0,
			build_once: None,
			tex_dds: st.get("tex_fmt") == Some("dds"),
			other_files: st.get("other_files") == Some("1"),
			keep_failed: st.get("keep_failed") != Some("0"),
			tables_json: st.get("table_fmt") == Some("json"),
			skip_check: false,
			last_build: None,
			build_ready: false,
			det_cache: Default::default(),
			settings: st,
		}
	}

	fn save_settings(&mut self) {
		let st = &mut self.settings;
		st.set("game", &self.src);
		st.set("mod_name", &self.mod_name);
		st.set("mask", &self.mask);
		st.set("page", if self.page == 1 { "1" } else { "0" });
		st.set("tex_fmt", if self.tex_dds { "dds" } else { "png" });
		st.set("other_files", if self.other_files { "1" } else { "0" });
		st.set("keep_failed", if self.keep_failed { "1" } else { "0" });
		st.set("table_fmt", if self.tables_json { "json" } else { "xlsx" });
		st.set("mode", if self.mode == 1 { "mod" } else { "rebuild" });
		st.set("title", &self.game_sel.to_string());
		st.set("dark", if self.pal.dark { "1" } else { "0" });
		st.set("lang", if ui_en() { "en" } else { "ru" });
		match sel_game(self.game_sel) {
			Some(g) => {
				st.set(&format!("proj_{}", g.key()), &self.proj);
				st.set(&format!("out_{}", g.key()), &self.out);
			}
			None => {
				st.set("proj", &self.proj);
				st.set("out", &self.out);
			}
		}
		st.set("unpack_speed", &format!("{:.0}", crate::kai_pick::speed()));
		st.set("build_speed", &format!("{:.0}", crate::kai_pick::build_speed()));
		st.save();
	}

	fn on_game_changed(&mut self, old: usize) {
		let new = self.game_sel;
		{
			let st = &mut self.settings;
			match sel_game(old) {
				Some(g) => {
					st.set(&format!("proj_{}", g.key()), &self.proj);
					st.set(&format!("out_{}", g.key()), &self.out);
				}
				None => {
					st.set("proj", &self.proj);
					st.set("out", &self.out);
				}
			}
		}
		let g = sel_game(new);
		let (kp, ko) = match g {
			Some(g) => (format!("proj_{}", g.key()), format!("out_{}", g.key())),
			None => ("proj".into(), "out".into()),
		};
		let horizon = g == Some(Game::Horizon);
		let st = &self.settings;
		self.proj = st.get(&kp).map(|s| s.to_string()).or_else(|| if horizon { st.get("proj").map(|s| s.to_string()) } else { None }).unwrap_or_default();
		self.out = st.get(&ko).map(|s| s.to_string()).or_else(|| if horizon { st.get("out").map(|s| s.to_string()) } else { None }).unwrap_or_default();
		self.save_settings();
	}

	fn sources(&self) -> (PathBuf, Vec<PathBuf>) {
		let items: Vec<PathBuf> = self.src.split(';').map(|x| x.trim().trim_matches('"').to_string()).filter(|x| !x.is_empty()).map(PathBuf::from).collect();
		if items.len() == 1 && !items[0].is_file() {
			return (items[0].clone(), Vec::new());
		}
		let files: Vec<PathBuf> = items.into_iter().filter(|p| !p.is_dir()).collect();
		let mut o = Options { game_dir: PathBuf::new(), sources: files.clone(), rebuild: true, proj_dir: PathBuf::new(), out_dir: PathBuf::new(), mod_name: String::new(), use_mods: true, scena: true, tables: true, textures: true, tex_dds: false, tables_json: false, check: false, other: true, keep_failed: true, mask: String::new(), only_changed: true, overwrite_edited: false, game: None, include: None, build_only: None };
		o.game_dir = kjobs::source_base(&o);
		(o.game_dir, files)
	}

	fn options(&self) -> Options {
		let (game_dir, sources) = self.sources();
		Options {
			game_dir,
			sources,
			rebuild: self.mode == 0,
			proj_dir: PathBuf::from(self.proj.trim()),
			out_dir: PathBuf::from(self.out.trim()),
			mod_name: self.mod_name.trim().to_string(),
			use_mods: true,
			scena: true,
			tables: true,
			textures: true,
			tex_dds: self.tex_dds,
			tables_json: self.tables_json,
			check: !self.skip_check,
			other: true,
			keep_failed: self.keep_failed,
			mask: self.mask.trim().to_string(),
			only_changed: true,
			overwrite_edited: false,
			game: sel_game(self.game_sel),
			include: self.sel_items().and_then(|v| Picker::selected_keys(v)),
			build_only: self.build_once.clone(),
		}
	}

	fn src_dir(&self) -> Option<String> {
		let t = self.src.trim().trim_matches('"');
		if t.is_empty() || t.contains(';') {
			return None;
		}
		std::path::Path::new(t).is_dir().then(|| t.to_string())
	}

	fn sel_items(&self) -> Option<&Vec<PItem>> {
		let d = self.src_dir()?;
		match &self.dec_sel {
			Some((dir, items)) if *dir == d => Some(items),
			_ => None,
		}
	}

	fn open_pick(&mut self, then_start: bool, ctx: &egui::Context) {
		let Some(d) = self.src_dir() else { return };
		let mut pk = match self.sel_items() {
			Some(items) => Picker::with_items(PMode::Decompile, &d, items.clone(), then_start, false),
			None => Picker::open(PMode::Decompile, &d, then_start, false, ctx),
		};
		pk.tex_dds = self.tex_dds;
		pk.other_files = self.other_files;
		pk.keep_failed = self.keep_failed;
		pk.tables_json = self.tables_json;
		self.pick = Some(pk);
	}

	fn detected(&self) -> Option<Game> {
		if self.src.trim().is_empty() {
			return None;
		}
		if let Some((src, g)) = self.det_cache.borrow().as_ref()
			&& *src == self.src {
				return *g;
			}
		let (dir, files) = self.sources();
		let d = if files.is_empty() { dir } else { files[0].parent().map(|p| p.to_path_buf()).unwrap_or(dir) };
		let g = calvard_tool::kai::game::detect(&d);
		*self.det_cache.borrow_mut() = Some((self.src.clone(), g));
		g
	}

	fn push_log(&mut self, text: &str, kind: Kind) {
		self.log.push(LogLine { time: now(), text: text.to_string(), kind });
		self.log_to_end = true;
	}

	fn classify(s: &str) -> Kind {
		let t = s.trim_start();
		if t.starts_with(tr("Готово", "Done")) {
			Kind::Done
		} else if t.starts_with('✖') || t.starts_with(tr("Остановлено", "Stopped")) || t.starts_with("⚠") || t.contains(tr("ошибка", "error")) || t.contains(tr("Ошибка", "Error")) {
			Kind::Warn
		} else if t.starts_with(tr("Декомпиляция (", "Decompiling (")) || t.starts_with(tr("Игра:", "Game:")) || t.starts_with(tr("Сборка", "Building")) && !t.starts_with(tr("Сборка завершена", "Build finished")) {
			Kind::Start
		} else {
			Kind::Normal
		}
	}

	fn start(&mut self, op: Op, ctx: &egui::Context, confirmed: bool) {
		if self.shared.is_some() {
			return;
		}
		let compile = op == Op::Compile;
		if !compile && !confirmed && self.src_dir().is_some() && self.sel_items().is_none() {
			self.open_pick(true, ctx);
			return;
		}
		if compile && !confirmed && !self.build_ready {
			let proj = self.proj.trim().to_string();
			if !proj.is_empty() && std::path::Path::new(&proj).is_dir() {
				self.pick = Some(Picker::open(PMode::Build, &proj, true, self.mode == 1, ctx));
				return;
			}
		}
		let mut o = self.options();
		if !compile {
			o.other = self.other_files;
		}
		if !compile && o.sources.is_empty() && !o.game_dir.is_dir() {
			self.tab = 1;
			self.push_log(tr("Укажите папку игры или файлы.", "Choose the game folder or files."), Kind::Warn);
			return;
		}
		if let Some(f) = o.sources.iter().find(|f| !f.is_file()) {
			self.dialog = Dialog::Info(lformat!("Файл не найден:\n{}", "File not found:\n{}", f.display()));
			return;
		}
		if o.proj_dir.as_os_str().is_empty() || (compile && o.out_dir.as_os_str().is_empty()) {
			self.dialog = Dialog::Info(tr("Укажите папку для правки и папку для собранного.", "Choose the project folder and the output folder.").into());
			return;
		}
		if !compile && !confirmed {
			let (n, size) = kjobs::plan(&o);
			let gb = size as f64 / 1073741824.0;
			let free = kjobs::free_space(&o.proj_dir).map(|f| f as f64 / 1073741824.0);
			if free.map(|f| f < gb).unwrap_or(false) {
				self.dialog = Dialog::Ask(
					lformat!("НЕ ХВАТИТ МЕСТА НА ДИСКЕ.\n\nБудет распаковано {n} файлов — нужно примерно {gb:.1} ГБ, а на диске папки для правки свободно {:.1} ГБ.\n\nВыберите папку для правки на диске, где больше места, или освободите место.\nЕсли вам нужен только перевод, укажите в «Папка игры или файлы» папку с русификатором — это намного меньше.\n\nВсё равно начать?", "NOT ENOUGH DISK SPACE.\n\n{n} files will be unpacked, which needs about {gb:.1} GB, but the project folder's disk has {:.1} GB free.\n\nChoose a project folder on a disk with more space, or free some space.\nIf you only need the text, select the translation mod's folder in 'Game folder or files': it is much smaller.\n\nStart anyway?",
						free.unwrap_or(0.0)
					),
					Action::Start(op),
				);
				return;
			} else if gb >= 1.0 {
				self.dialog = Dialog::Ask(
					lformat!("Будет распаковано {n} файлов, примерно {gb:.1} ГБ{}.\nЭто может занять много времени.\n\nПродолжить?", "{n} files will be unpacked, about {gb:.1} GB{}.\nThis may take a long time.\n\nContinue?", free.map(|f| lformat!(" (свободно {f:.1} ГБ)", " ({f:.1} GB free)")).unwrap_or_default()),
					Action::Start(op),
				);
				return;
			}
		}
		let out_is_game = !o.game_dir.as_os_str().is_empty() && same_dir(&o.out_dir, &o.game_dir);
		if compile && o.rebuild && out_is_game {
			self.dialog = Dialog::Info(tr("При пересборке архивов «Куда сохранять собранное» должна быть другой папкой, не папкой игры: архивы собираются из оригиналов игры. Потом скопируйте собранное в игру с заменой (оригиналы сохраните).", "When rebuilding archives, the 'Output folder' must not be the game folder, because the archives are built from the game's originals. Copy the result into the game afterwards, replacing the files (keep the originals).").into());
			return;
		}
		let dst_mod = o.out_dir.join("mods").join(format!("{}.p3a", kjobs::mod_file_name(&o.mod_name)));
		if compile && !confirmed && !o.rebuild && dst_mod.exists() && out_is_game {
			self.dialog = Dialog::Ask(lformat!("Файл {} уже есть в папке игры и будет заменён.\nСделайте резервную копию, если её нет.\n\nПродолжить?", "The file {} already exists in the game folder and will be replaced.\nMake a backup if you don't have one.\n\nContinue?", dst_mod.display()), Action::Start(op));
			return;
		}
		self.save_settings();
		self.log.clear();
		let _ = std::fs::write(log_path(), "\u{FEFF}");
		self.tab = 1;
		self.status = if compile { tr("Сборка…", "Building…").into() } else { tr("Декомпиляция…", "Decompiling…").into() };
		let shared = Arc::new(Shared::default());
		self.shared = Some(shared.clone());
		self.op = Some(op);
		self.op_start = Instant::now();
		self.last_build = o.build_only.clone();
		self.skip_check = false;
		self.build_once = None;
		self.build_ready = false;
		let est: u64 = if compile { self.build_est } else { self.sel_items().map(|v| v.iter().filter(|i| i.on).map(|i| i.unpacked).sum()).unwrap_or(0) };
		let sink = GuiSink { shared, ctx: ctx.clone() };
		std::thread::spawn(move || {
			let start = Instant::now();
			let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| if compile { kjobs::compile(&o, &sink) } else { kjobs::decompile(&o, &sink) }));
			if res.is_err() {
				jobs::Sink::log(&sink, tr("✖ Внутренняя ошибка программы.", "✖ Internal program error."));
			}
			let cancelled = sink.shared.cancel.load(Ordering::SeqCst);
			if cancelled {
				jobs::Sink::log(&sink, tr("Остановлено пользователем.", "Stopped by the user."));
			}
			jobs::Sink::log(&sink, &lformat!("Время: {:.1} с", "Time: {:.1} s", start.elapsed().as_secs_f32()));
			if let (true, false, Ok(s)) = (compile, cancelled, &res) {
				let secs = start.elapsed().as_secs_f64();
				if est > 0 && secs >= 10.0 && s.ok > 0 {
					crate::kai_pick::set_build_speed(est as f64 / secs);
				}
			}
			if let (false, false, Ok(s)) = (compile, cancelled, &res) {
				let secs = start.elapsed().as_secs_f64();
				if est > 0 && secs >= 30.0 && s.same * 4 <= s.ok {
					crate::kai_pick::set_speed(est as f64 / secs);
				}
			}
			if !cancelled && start.elapsed().as_secs() >= 10 {
				let what = if compile { tr("сборка", "build") } else { tr("декомпиляция", "decompile") };
				let text = match &res {
					Ok(s) => lformat!("Готово: успешно {}, ошибок {}{}", "Done: {} ok, {} errors{}", s.ok, s.failed, if s.warnings > 0 { lformat!(", предупреждений {}", ", {} warnings", s.warnings) } else { String::new() }),
					Err(_) => tr("Внутренняя ошибка программы — см. журнал", "Internal program error, see the log").into(),
				};
				crate::gui_common::notify_done(&lformat!("CalvardTool: {what} завершена", "CalvardTool: {what} finished"), &text);
			}
			if let Ok(s) = &res {
				*sink.shared.check.lock().unwrap() = s.check.clone();
			}
			sink.shared.finished.store(true, Ordering::SeqCst);
			sink.ctx.request_repaint();
		});
	}

	fn poll(&mut self) {
		let Some(sh) = self.shared.clone() else { return };
		let lines: Vec<String> = std::mem::take(&mut *sh.log.lock().unwrap());
		for l in lines {
			for part in l.lines() {
				let k = Self::classify(part);
				self.push_log(part, k);
			}
		}
		let (d, t) = (sh.done.load(Ordering::SeqCst), sh.total.load(Ordering::SeqCst));
		if t > 0 && !sh.finished.load(Ordering::SeqCst) {
			let what = if self.op == Some(Op::Compile) { tr("Сборка", "Building") } else { tr("Декомпиляция", "Decompiling") };
			let left = if d >= 3 && d < t { lformat!(" · осталось {}", " · {} left", crate::kai_pick::fmt_time(self.op_start.elapsed().as_secs_f64() / d as f64 * (t - d) as f64)) } else { String::new() };
			let cur = sh.current.lock().unwrap().clone();
			self.status = lformat!("{what}: {d} из {t} файлов{left}{}", "{what}: {d} of {t} files{left}{}", if cur.is_empty() { String::new() } else { format!(" · {cur}") });
		}
		if sh.finished.load(Ordering::SeqCst) {
			let cancelled = sh.cancel.load(Ordering::SeqCst);
			let check = std::mem::take(&mut *sh.check.lock().unwrap());
			if !check.is_empty() {
				let mut list: Vec<String> = check.iter().take(12).map(|s| format!("· {s}")).collect();
				if check.len() > 12 {
					list.push(lformat!("… и ещё {} (все — в журнале)", "… and {} more (all of them are in the log)", check.len() - 12));
				}
				self.dialog = Dialog::Ask(lformat!("В переводе сломаны служебные коды или подстановки — {}. Игра может показать не то или упасть.\n\n{}\n\nИсправьте их (подробно — вкладка «Проверки» Сверки) или соберите всё равно.\n\nСобрать всё равно?", "Service codes or placeholders are broken in the translation: {}. The game may show the wrong text or crash.\n\n{}\n\nFix them (details are in the Checks tab of CalvardCompare) or build anyway.\n\nBuild anyway?", check.len(), list.join("\n")), Action::ForceBuild);
			}
			let refused = !cancelled && self.log.iter().any(|l| l.text.trim_start().starts_with('✖')) && !self.log.iter().any(|l| matches!(l.kind, Kind::Done));
			self.status = if cancelled {
				tr("Остановлено", "Stopped").into()
			} else if refused {
				tr("Не выполнено — причина в журнале", "Not done: see the log for the reason").into()
			} else if self.op == Some(Op::Compile) {
				tr("Сборка завершена", "Build finished").into()
			} else {
				tr("Декомпиляция завершена — правьте файлы в папке проекта", "Decompile finished: edit the files in the project folder").into()
			};
			if !cancelled && !refused {
				self.last_done = self.op;
			}
			self.shared = None;
			self.op = None;
			self.save_settings();
		}
	}

	fn progress(&self) -> Option<f32> {
		let sh = self.shared.as_ref()?;
		let (d, t) = (sh.done.load(Ordering::SeqCst), sh.total.load(Ordering::SeqCst));
		Some(if t == 0 { 0.0 } else { d as f32 / t as f32 })
	}

	fn flash(&mut self, s: &str) {
		self.flash = Some((s.to_string(), Instant::now()));
	}

	fn mask_field(&mut self, ui: &mut egui::Ui, p: &Pal, mr: Rect, busy: bool) {
		let mut r = ui.new_child(egui::UiBuilder::new().max_rect(mr));
		r.painter().rect(mr, 8, p.surface, Stroke::new(1.0, p.div), StrokeKind::Inside);
		let mut cr = r.new_child(egui::UiBuilder::new().max_rect(mr.shrink2(Vec2::new(12.0, 0.0))).layout(egui::Layout::left_to_right(egui::Align::Center)));
		let resp = cr
			.add(egui::TextEdit::singleline(&mut self.mask).frame(false).font(f_body(13.0)).desired_width(mr.width() - 24.0).interactive(!busy).hint_text(tr("все файлы", "all files")).margin(egui::Margin::ZERO))
			.on_hover_text(tr("Только эти файлы, через запятую: c0100, t_item, c01*\nПусто — все файлы", "Only these files, comma-separated: c0100, t_item, c01*\nEmpty means all files"));
		if resp.lost_focus() {
			self.save_settings();
		}
	}

	fn ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
		let p = self.pal;
		let full = ui.max_rect();
		let x0 = full.min.x + 22.0;
		let x1 = full.max.x - 22.0;
		let w = x1 - x0;
		let busy = self.shared.is_some();
		let mut y = full.min.y + 18.0;

		y -= 8.0;
		if page_tabs(ui, &p, x0, x1, y, &[tr("Разборка", "Decompile"), tr("Сборка", "Building")], &mut self.page) {
			self.save_settings();
		}
		y += 34.0 + 12.0;
		let label_h = 16.0;
		let prog = self.progress();
		let bh = 76.0;
		let caption = |ui: &egui::Ui, x: f32, y: f32, t: &str, c: Color32| {
			ui.painter().galley(Pos2::new(x, y), ui.fonts(|f| f.layout_job(caps_job(t, c))), c);
		};
		let det = self.detected();
		if self.page == 0 {
			caption(ui, x0, y, tr("Папка игры или файлы", "Game folder or files"), p.muted);
			let (right, rc) = if let Some(g) = sel_game(self.game_sel) {
				(lformat!("Выбрано вручную: {}", "Chosen manually: {}", g.name()), p.accent)
			} else if let Some(g) = det {
				(lformat!("Определено по папке: {}", "Detected from the folder: {}", g.name()), p.accent)
			} else if !self.src.trim().is_empty() {
				(tr("Игра не определена — будет Trails beyond the Horizon", "Game not detected: Trails beyond the Horizon will be used").to_string(), p.muted)
			} else {
				(tr("Игра определится по папке: kuro.exe — Daybreak, kuro2.exe — Daybreak II, иначе Horizon", "The game is detected from the folder: kuro.exe is Daybreak, kuro2.exe is Daybreak II, otherwise Horizon").to_string(), p.muted)
			};
			ui.painter().text(Pos2::new(x1, y + 6.0), Align2::RIGHT_CENTER, right, f_body(11.5), rc);
			y += label_h + 5.0;
			let empty = self.src.trim().is_empty();
			let mut r = ui.new_child(egui::UiBuilder::new().max_rect(Rect::from_min_size(Pos2::new(x0, y), Vec2::new(w, 38.0))));
			let (_, browse, _) = path_field(&mut r, &p, "src", &mut self.src, 38.0, tr("Перетащите сюда папку игры — или нажмите «Обзор…»", "Drop the game folder here, or click 'Browse…'"), tr("Обзор…", "Browse…"), empty, !busy);
			if browse {
				let (dir, _) = self.sources();
				if let Some(d) = pick_folder(&dir.to_string_lossy()) {
					self.src = d.display().to_string();
					self.save_settings();
					self.dec_sel = None;
					self.open_pick(false, ctx);
				}
			}
			y += 38.0 + 12.0;

			caption(ui, x0, y, tr("Куда сохранять для правки", "Project folder (for editing)"), p.muted);
			let mut r = ui.new_child(egui::UiBuilder::new().max_rect(Rect::from_min_size(Pos2::new(x0, y + label_h + 5.0), Vec2::new(w, 36.0))));
			let (_, clicked, _) = path_field(&mut r, &p, "proj", &mut self.proj, 36.0, "", tr("Обзор…", "Browse…"), false, !busy);
			if clicked
				&& let Some(d) = pick_folder(&self.proj) {
					self.proj = d.display().to_string();
					self.save_settings();
				}
			y += label_h + 5.0 + 36.0 + 12.0;

			let cw = (w - 28.0) / 3.0;
			caption(ui, x0, y, tr("Игра", "Game"), p.muted);
			caption(ui, x0 + cw + 14.0, y, tr("Маска файлов", "File mask"), p.muted);
			caption(ui, x0 + 2.0 * (cw + 14.0), y, tr("Что разбирать", "What to decompile"), p.muted);
			let fy = y + label_h + 5.0;
			{
				let mut r = ui.new_child(egui::UiBuilder::new().max_rect(Rect::from_min_size(Pos2::new(x0, fy), Vec2::new(cw, 36.0))));
				let items: Vec<String> = game_items().iter().map(|s| s.to_string()).collect();
				let old = self.game_sel;
				if combo(&mut r, &p, "game", &items, &mut self.game_sel, cw, 36.0, 8, f_body(13.5)) {
					self.on_game_changed(old);
					if let Some(g) = sel_game(self.game_sel) {
						self.flash(&lformat!("Папки проекта и сборки — для {}", "Project and output folders: for {}", g.name()));
					}
				}
			}
			self.mask_field(ui, &p, Rect::from_min_size(Pos2::new(x0 + cw + 14.0, fy), Vec2::new(cw, 36.0)), busy);
			{
				let has = self.src_dir().is_some();
				let t = match self.sel_items() {
					Some(v) => lformat!("Файлы: {} из {}", "Files: {} of {}", v.iter().filter(|i| i.on).count(), v.len()),
					None => tr("Файлы…", "Files…").to_string(),
				};
				let mut bu = ui.new_child(egui::UiBuilder::new().max_rect(Rect::from_min_size(Pos2::new(x0 + 2.0 * (cw + 14.0), fy), Vec2::new(cw, 36.0))));
				if small_button(&mut bu, &p, &t, Some("check"), 36.0, has && !busy, p.accent).on_hover_text(tr("Выбрать галочками, что разбирать", "Tick what to decompile")).clicked() {
					self.open_pick(false, ctx);
				}
			}
			y = fy + 36.0 + 18.0;
			let dec_part = self.sel_items().and_then(|v| {
				let n = v.iter().filter(|i| i.on).count();
				(n < v.len()).then_some((n, v.len()))
			});
			let title = if self.op == Some(Op::Decompile) { tr("Декомпиляция…", "Decompiling…") } else if dec_part.is_some() { tr("Декомпилировать выбранное", "Decompile selected") } else { tr("Декомпилировать всё", "Decompile all") };
			let subt = match dec_part {
				Some((n, m)) => lformat!("{n} из {m} → папка проекта", "{n} of {m} → project folder"),
				None => tr("папка игры → папка для правки", "game folder → project folder").to_string(),
			};
			if action_button(ui, &p, Rect::from_min_size(Pos2::new(x0, y), Vec2::new(w, bh)), "dec", "archive", title, &subt, busy, self.op == Some(Op::Decompile), prog) {
				self.start(Op::Decompile, ctx, false);
			}
		} else {
			caption(ui, x0, y, tr("Папка для сборки", "Project folder (to build)"), p.muted);
			let mut r = ui.new_child(egui::UiBuilder::new().max_rect(Rect::from_min_size(Pos2::new(x0, y + label_h + 5.0), Vec2::new(w, 36.0))));
			let (_, clicked, _) = path_field(&mut r, &p, "proj", &mut self.proj, 36.0, "", tr("Обзор…", "Browse…"), false, !busy);
			if clicked
				&& let Some(d) = pick_folder(&self.proj) {
					self.proj = d.display().to_string();
					self.save_settings();
				}
			y += label_h + 5.0 + 36.0 + 12.0;

			caption(ui, x0, y, tr("Куда сохранять собранное", "Output folder"), p.muted);
			let mut r = ui.new_child(egui::UiBuilder::new().max_rect(Rect::from_min_size(Pos2::new(x0, y + label_h + 5.0), Vec2::new(w, 36.0))));
			let (_, clicked, _) = path_field(&mut r, &p, "out", &mut self.out, 36.0, "", tr("Обзор…", "Browse…"), false, !busy);
			if clicked
				&& let Some(d) = pick_folder(&self.out) {
					self.out = d.display().to_string();
					self.save_settings();
				}
			y += label_h + 5.0 + 36.0 + 12.0;

			let c3 = 170.0;
			let c4 = 170.0;
			let c2 = w - c3 - c4 - 28.0;
			caption(ui, x0, y, tr("Как собирать", "Build mode"), p.muted);
			let mod_alpha = if self.mode == 1 { 1.0 } else { 0.45 };
			caption(ui, x0 + c2 + 14.0, y, tr("Имя мода", "Mod name"), p.muted.gamma_multiply(mod_alpha));
			caption(ui, x0 + c2 + c3 + 28.0, y, tr("Маска файлов", "File mask"), p.muted);
			let fy = y + label_h + 5.0;
			{
				let mut r = ui.new_child(egui::UiBuilder::new().max_rect(Rect::from_min_size(Pos2::new(x0, fy), Vec2::new(c2, 36.0))));
				if segmented(&mut r, &p, "mode", &[tr("Пересобрать архивы игры", "Rebuild game archives"), tr("Одним модом в mods", "As one mod in mods")], &mut self.mode, Some(c2), 36.0, 8, f_body(13.5), &[tr("Каждая папка проекта — обратно в свой .p3a", "Each project folder goes back into its own .p3a"), tr("Все изменения в один mods\\<имя мода>.p3a", "All changes into one mods\\<mod name>.p3a")]) {
					self.save_settings();
				}
			}
			{
				let mr = Rect::from_min_size(Pos2::new(x0 + c2 + 14.0, fy), Vec2::new(c3, 36.0));
				let mut r = ui.new_child(egui::UiBuilder::new().max_rect(mr));
				let en = self.mode == 1 && !busy;
				r.set_opacity(mod_alpha);
				r.painter().rect(mr, 8, p.surface, Stroke::new(1.0, p.div), StrokeKind::Inside);
				let mut cr = r.new_child(egui::UiBuilder::new().max_rect(mr.shrink2(Vec2::new(12.0, 0.0))).layout(egui::Layout::left_to_right(egui::Align::Center)));
				let resp = cr.add(egui::TextEdit::singleline(&mut self.mod_name).frame(false).font(f_body(13.0)).desired_width(c3 - 24.0).interactive(en).margin(egui::Margin::ZERO));
				if resp.lost_focus() {
					self.save_settings();
				}
			}
			self.mask_field(ui, &p, Rect::from_min_size(Pos2::new(x0 + c2 + c3 + 28.0, fy), Vec2::new(c4, 36.0)), busy);
			y = fy + 36.0 + 18.0;
			let title = if self.op == Some(Op::Compile) { tr("Сборка…", "Building…") } else { tr("Собрать", "Build") };
			let subt = if self.mode == 0 { tr("Проект → изменённые архивы .p3a", "Project → changed .p3a archives").to_string() } else { lformat!("Проект → mods\\{}.p3a", "Project → mods\\{}.p3a", kjobs::mod_file_name(&self.mod_name)) };
			if action_button(ui, &p, Rect::from_min_size(Pos2::new(x0, y), Vec2::new(w, bh)), "build", "package", title, &subt, busy, self.op == Some(Op::Compile), prog) {
				self.start(Op::Compile, ctx, false);
			}
		}
		y += bh + 14.0;

		{
			let row = Rect::from_min_size(Pos2::new(x0, y), Vec2::new(w, 32.0));
			let mut r = ui.new_child(egui::UiBuilder::new().max_rect(row).layout(egui::Layout::right_to_left(egui::Align::Center)));
			r.spacing_mut().item_spacing.x = 10.0;
			if self.page == 1 {
				if small_button(&mut r, &p, tr("Открыть собранное", "Open output"), Some("folder"), 32.0, true, p.text).clicked() {
					open_folder(&self.out);
					self.flash(tr("Открыта папка собранного", "Output folder opened"));
				}
			} else if small_button(&mut r, &p, tr("Открыть проект", "Open project"), Some("folder"), 32.0, true, p.text).clicked() {
				open_folder(&self.proj);
				self.flash(tr("Открыта папка проекта", "Project folder opened"));
			}
			let paused = busy && self.shared.as_ref().is_some_and(|s| s.pause.load(Ordering::SeqCst));
			if small_button(&mut r, &p, tr("Стоп", "Stop"), Some("stop"), 32.0, busy, p.warn).clicked()
				&& let Some(s) = &self.shared {
					s.cancel.store(true, Ordering::SeqCst);
				}
			let tip = if paused { tr("Продолжить с того же места", "Continue from where it stopped") } else { tr("Приостановить: текущий файл доделается, следующий не начнётся", "Pause: the current file is finished, the next one does not start") };
			if small_button(&mut r, &p, if paused { tr("Продолжить", "Continue") } else { tr("Пауза", "Pause") }, Some(if paused { "play" } else { "pause" }), 32.0, busy, if paused { p.accent_strong } else { p.text }).on_hover_text(tip).clicked()
				&& let Some(s) = &self.shared {
					s.pause.store(!paused, Ordering::SeqCst);
				}
			let left = r.min_rect();
			let bar_w = (left.min.x - 12.0 - x0).max(50.0);
			let pr = prog.unwrap_or(if self.last_done.is_some() && !busy { 1.0 } else { 0.0 });
			let txt = Rect::from_min_size(Pos2::new(x0, y + 2.0), Vec2::new(bar_w, 16.0));
			paint_text_clipped(ui, Rect::from_min_max(txt.min, Pos2::new(txt.max.x - 60.0, txt.max.y)), if paused { tr("Пауза — нажмите «Продолжить»", "Paused: press Continue") } else { self.status.as_str() }, f_body(12.0), if paused { p.warn } else { p.muted }, Align2::LEFT_CENTER);
			let pct = if busy { format!("{}%", (pr * 100.0).round() as i32) } else if self.last_done.is_some() { tr("готово", "done").into() } else { String::new() };
			ui.painter().text(Pos2::new(x0 + bar_w, txt.center().y), Align2::RIGHT_CENTER, pct, f_body(12.0), p.muted);
			let bar = Rect::from_min_size(Pos2::new(x0, y + 22.0), Vec2::new(bar_w, 6.0));
			ui.painter().rect_filled(bar, 3, p.div2);
			if pr > 0.0 {
				ui.painter().rect_filled(Rect::from_min_size(bar.min, Vec2::new(bar_w * pr.clamp(0.0, 1.0), 6.0)), 3, if paused { p.warn } else { p.accent });
			}
		}
		y += 32.0 + 16.0;

		let bottom_h = 40.0;
		let panel = Rect::from_min_max(Pos2::new(x0, y), Pos2::new(x1, full.max.y - bottom_h));
		ui.painter().rect(panel, 12, Color32::TRANSPARENT, Stroke::new(1.0, p.div), StrokeKind::Inside);
		let head = Rect::from_min_size(panel.min, Vec2::new(panel.width(), 40.0));
		{
			let mut tx = head.min.x + 8.0;
			for (i, t) in [tr("Справка", "Help"), tr("Журнал", "Log")].iter().enumerate() {
				let tw = ui.fonts(|f| f.layout_no_wrap(t.to_string(), f_body(13.5), p.text).size().x) + 24.0;
				let tr = Rect::from_min_size(Pos2::new(tx, head.center().y - 14.0), Vec2::new(tw, 28.0));
				let resp = ui.interact(tr, ui.id().with(("tab", i)), Sense::click());
				if self.tab == i {
					ui.painter().rect_filled(tr, 7, p.sel);
				} else if resp.hovered() {
					ui.painter().rect_filled(tr, 7, p.div2);
				}
				ui.painter().text(tr.center(), Align2::CENTER_CENTER, *t, f_body(13.5), if self.tab == i { p.text } else { p.muted });
				if resp.clicked() {
					self.tab = i;
				}
				tx += tw + 4.0;
			}
			ui.painter().text(Pos2::new(head.max.x - 14.0, head.center().y), Align2::RIGHT_CENTER, tr("Настройки и журнал: CalvardTool.ini, CalvardTool.log", "Settings and log: CalvardTool.ini, CalvardTool.log"), f_body(12.0), p.muted);
			ui.painter().line_segment([Pos2::new(panel.min.x + 1.0, head.max.y), Pos2::new(panel.max.x - 1.0, head.max.y)], Stroke::new(1.0, p.div));
		}
		let body = Rect::from_min_max(Pos2::new(panel.min.x + 1.0, head.max.y + 1.0), Pos2::new(panel.max.x - 1.0, panel.max.y - 1.0));
		if self.tab == 1 {
			ui.painter().rect_filled(body, egui::CornerRadius { nw: 0, ne: 0, sw: 11, se: 11 }, p.log);
		}
		let mut r = ui.new_child(egui::UiBuilder::new().max_rect(body.shrink2(Vec2::new(0.0, 0.0))));
		r.set_clip_rect(body);
		if self.tab == 0 {
			self.help(&mut r, &p);
		} else {
			self.log_view(&mut r, &p);
		}

		let br = Rect::from_min_max(Pos2::new(x0, full.max.y - bottom_h), Pos2::new(x1, full.max.y));
		let left = match sel_game(self.game_sel).or(det) {
			Some(g) => lformat!("Папки проекта и сборки — для {}", "Project and output folders: for {}", g.name()),
			None => tr("Перед сборкой сохраните оригиналы игры копией", "Back up the game's originals before building").to_string(),
		};
		let lg = ui.painter().layout_no_wrap(left, f_body(12.0), p.muted);
		let lw = lg.size().x;
		ui.painter().galley(Pos2::new(br.min.x, br.center().y - lg.size().y / 2.0), lg, p.muted);
		if let Some((m, t)) = &self.flash {
			if t.elapsed().as_secs_f32() < 1.8 {
				ui.painter().text(Pos2::new(br.min.x + lw + 16.0, br.center().y), Align2::LEFT_CENTER, m, f_body(12.0), p.ok_strong);
				ctx.request_repaint_after(std::time::Duration::from_millis(200));
			} else {
				self.flash = None;
			}
		}
		let mut r = ui.new_child(egui::UiBuilder::new().max_rect(br).layout(egui::Layout::right_to_left(egui::Align::Center)));
		if small_button(&mut r, &p, if ui_en() { "RU" } else { "EN" }, Some("languages"), 26.0, true, p.text).on_hover_text(tr("Switch to English", "Переключить на русский")).clicked() {
			let idle = self.status == tr("Готов к работе", "Ready");
			UI_EN.store(!ui_en(), std::sync::atomic::Ordering::Relaxed);
			self.save_settings();
			if idle {
				self.status = tr("Готов к работе", "Ready").into();
			}
		}
		if theme_button(&mut r, &p, 26.0).clicked() {
			self.pal = Pal::get(!self.pal.dark);
			apply_visuals(ctx, &self.pal);
			self.save_settings();
		}

	}

	fn help(&self, ui: &mut egui::Ui, p: &Pal) {
		let r = ui.max_rect();
		let inner = r.shrink2(Vec2::new(20.0, 16.0));
		let mut c = ui.new_child(egui::UiBuilder::new().max_rect(inner));
		egui::ScrollArea::vertical().id_salt("help").auto_shrink([false, false]).show(&mut c, |ui| {
			ui.label(egui::RichText::new(tr("CalvardTool — декомпиляция и сборка The Legend of Heroes: Trails through Daybreak, Daybreak II, Trails beyond the Horizon. Игра определяется сама по папке (kuro.exe, kuro2.exe…) или выберите её в списке «Игра» — у каждой игры свои папки проекта.", "CalvardTool decompiles and builds The Legend of Heroes: Trails through Daybreak, Daybreak II and Trails beyond the Horizon. The game is detected from the folder (kuro.exe, kuro2.exe…), or choose it in the 'Game' list. Each game has its own project folders.")).font(f_body(14.0)).color(p.muted));
			ui.add_space(14.0);
			let cards = [
				(tr("1 · Папка игры или файлы", "1 · Game folder or files"), tr("«Обзор…» → папка игры или русификатора (или перетащите её в окно). Программа просканирует её и откроет окно «Что разбирать»: отметьте галочками нужное. Открыть окно снова — кнопка «Файлы» справа. «Маска файлов» — только нужные файлы: c0100, t_item, c01* (пусто — все).", "'Browse…' → the game folder or a translation mod's folder (or drop it on the window). The program scans it and opens the 'What to decompile' window: tick what you need. To reopen that window, use the 'Files' button on the right. 'File mask' limits the work to specific files: c0100, t_item, c01* (empty means all).")),
				(tr("2 · Декомпилировать", "2 · Decompile"), tr("Разбирается то, что отмечено, — в «Куда сохранять для правки»: диалоги в .json, таблицы в .xlsx, текстуры в .png или .dds (формат — на втором шаге окна «Что разбирать»). Уже изменённые вами файлы не перезаписываются.", "The ticked items are unpacked into the 'Project folder': dialogues to .json, tables to .xlsx, textures to .png or .dds (the format is chosen in the second step of 'What to decompile'). Files you have already edited are not overwritten.")),
				(tr("3 · Собрать", "3 · Build"), tr("Откроется окно «Что собрать» — отметьте папки, видно, где сколько изменено. Собираются только изменённые файлы — в «Куда сохранять собранное». Скопируйте собранное в папку игры с заменой (оригиналы сохраните).", "The 'What to build' window opens: tick the folders and see how many files changed in each. Only changed files are built, into the 'Output folder'. Copy the result into the game folder, replacing the files (keep the originals).")),
			];
			let cards = if self.page == 0 { &cards[..2] } else { &cards[2..] };
			let n = cards.len() as f32;
			let cw = (ui.available_width() - (n - 1.0) * 12.0) / n;
			let h = cards
				.iter()
				.map(|(_, t)| {
					let mut job = egui::text::LayoutJob::single_section(t.to_string(), egui::TextFormat { font_id: f_body(13.5), color: p.text, line_height: Some(20.0), ..Default::default() });
					job.wrap.max_width = cw - 28.0;
					ui.fonts(|f| f.layout_job(job)).size().y
				})
				.fold(0.0f32, f32::max)
				+ 24.0 + 30.0;
			let (row, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), h), Sense::hover());
			for (i, (t, txt)) in cards.iter().enumerate() {
				let cr = Rect::from_min_size(Pos2::new(row.min.x + i as f32 * (cw + 12.0), row.min.y), Vec2::new(cw, h));
				ui.painter().rect(cr, 10, Color32::TRANSPARENT, Stroke::new(1.0, p.div), StrokeKind::Inside);
				ui.painter().text(Pos2::new(cr.min.x + 14.0, cr.min.y + 20.0), Align2::LEFT_CENTER, *t, f_semi(15.0), p.accent);
				let mut job = egui::text::LayoutJob::single_section(txt.to_string(), egui::TextFormat { font_id: f_body(13.5), color: p.text, line_height: Some(20.0), ..Default::default() });
				job.wrap.max_width = cw - 28.0;
				let g = ui.fonts(|f| f.layout_job(job));
				ui.painter().galley(Pos2::new(cr.min.x + 14.0, cr.min.y + 38.0), g, p.text);
			}
		});
	}

	fn log_view(&mut self, ui: &mut egui::Ui, p: &Pal) {
		let r = ui.max_rect();
		let inner = r.shrink2(Vec2::new(16.0, 10.0));
		let mut c = ui.new_child(egui::UiBuilder::new().max_rect(inner));
		let row_h = 12.5 * 1.65;
		let n = self.log.len();
		let mut sa = egui::ScrollArea::vertical().id_salt("log").auto_shrink([false, false]).stick_to_bottom(true);
		if self.log_to_end {
			sa = sa.vertical_scroll_offset((n as f32 * row_h - inner.height()).max(0.0));
			self.log_to_end = false;
		}
		c.spacing_mut().item_spacing.y = 0.0;
		sa.show_rows(&mut c, row_h, n, |ui, range| {
			ui.spacing_mut().item_spacing.y = 0.0;
			for i in range {
				let l = &self.log[i];
				let (rr, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), row_h), Sense::hover());
				ui.painter().text(Pos2::new(rr.min.x, rr.center().y), Align2::LEFT_CENTER, &l.time, f_mono(12.5), p.muted);
				let col = match l.kind {
					Kind::Start => p.accent,
					Kind::Done => p.ok_strong,
					Kind::Warn => p.warn,
					Kind::Normal => p.text,
				};
				paint_text_clipped(ui, Rect::from_min_max(Pos2::new(rr.min.x + 72.0, rr.min.y), rr.max), &l.text, f_mono(12.5), col, Align2::LEFT_CENTER);
			}
		});
		if n == 0 {
			ui.painter().text(r.center(), Align2::CENTER_CENTER, tr("Журнал пуст — запустите декомпиляцию или сборку", "The log is empty: run a decompile or a build"), f_body(13.0), p.muted);
		}
	}
}

impl eframe::App for App {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		self.poll();
		if self.first_frame {
			self.first_frame = false;
		}
		let p = self.pal;
		let hovering = ctx.input(|i| !i.raw.hovered_files.is_empty());
		let dropped: Vec<PathBuf> = ctx.input(|i| i.raw.dropped_files.iter().filter_map(|f| f.path.clone()).collect());
		if !dropped.is_empty() && self.shared.is_none() {
			self.src = dropped.iter().map(|x| x.display().to_string()).collect::<Vec<_>>().join("; ");
			self.save_settings();
			self.dec_sel = None;
			self.page = 0;
			self.open_pick(false, ctx);
		}
		egui::CentralPanel::default().frame(egui::Frame::new().fill(p.bg)).show(ctx, |ui| {
			self.ui(ui, ctx);
		});
		if hovering {
			egui::Area::new(egui::Id::new("drop")).order(egui::Order::Foreground).fixed_pos(Pos2::ZERO).show(ctx, |ui| {
				let r = ctx.screen_rect().shrink(8.0);
				ui.painter().rect_filled(r, 12, p.bg.gamma_multiply(0.86));
				dashed_rect(ui, r, p.accent);
				ui.painter().text(r.center() - Vec2::new(0.0, 14.0), Align2::CENTER_CENTER, tr("Отпустите, чтобы указать папку игры", "Drop to set the game folder"), f_semi(26.0), p.accent);
				ui.painter().text(r.center() + Vec2::new(0.0, 18.0), Align2::CENTER_CENTER, tr("Папка игры целиком или файлы .p3a", "The whole game folder or .p3a files"), f_body(14.0), p.muted);
			});
		}
		if let Some(pk) = &mut self.pick {
			let build = pk.mode == PMode::Build;
			if build && pk.empty() {
				self.pick = None;
				self.build_est = 0;
				self.build_ready = true;
				self.start(Op::Compile, ctx, false);
			} else {
				match pk.show(ctx, &p) {
					PEvent::None => {}
					PEvent::Cancel => self.pick = None,
					PEvent::Done => {
						let pk = self.pick.take().unwrap();
						if build {
							self.build_once = Picker::selected_keys(&pk.items);
							self.build_est = pk.out_estimate();
							self.build_ready = true;
							self.start(Op::Compile, ctx, false);
						} else {
							let n = pk.n_on();
							self.tex_dds = pk.tex_dds;
							self.other_files = pk.other_files;
							self.keep_failed = pk.keep_failed;
							self.tables_json = pk.tables_json;
							self.save_settings();
							self.dec_sel = Some((pk.dir.clone(), pk.items));
							self.flash(&lformat!("Выбрано для разборки: {n}", "Selected to decompile: {n}"));
							if pk.then_start {
								self.start(Op::Decompile, ctx, false);
							}
						}
					}
				}
			}
		}
		match show_dialog(ctx, &p, "CalvardTool", &mut self.dialog) {
			Some(Action::Start(op)) => self.start(op, ctx, true),
			Some(Action::ForceBuild) => {
				self.skip_check = true;
				self.build_once = self.last_build.clone();
				self.build_ready = true;
				self.start(Op::Compile, ctx, true);
			}
			None => {}
		}
		if self.shared.is_some() {
			ctx.request_repaint_after(std::time::Duration::from_millis(100));
		}
	}

	fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
		self.save_settings();
		if let Some(s) = &self.shared {
			s.cancel.store(true, Ordering::SeqCst);
			let t = Instant::now();
			while !s.finished.load(Ordering::SeqCst) && t.elapsed().as_secs() < 30 {
				std::thread::sleep(std::time::Duration::from_millis(50));
			}
		}
	}
}

fn same_dir(a: &std::path::Path, b: &std::path::Path) -> bool {
	if a == b {
		return true;
	}
	match (std::fs::canonicalize(a), std::fs::canonicalize(b)) {
		(Ok(x), Ok(y)) => x == y,
		_ => {
			let norm = |p: &std::path::Path| std::path::absolute(p).unwrap_or(p.to_path_buf()).to_string_lossy().replace('/', "\\").trim_end_matches('\\').to_lowercase();
			norm(a) == norm(b)
		}
	}
}

fn dashed_rect(ui: &egui::Ui, r: Rect, c: Color32) {
	let s = Stroke::new(2.0, c);
	let p = ui.painter();
	let k = 12.0;
	let sides = [
		(Pos2::new(r.min.x + k, r.min.y), Pos2::new(r.max.x - k, r.min.y)),
		(Pos2::new(r.min.x + k, r.max.y), Pos2::new(r.max.x - k, r.max.y)),
		(Pos2::new(r.min.x, r.min.y + k), Pos2::new(r.min.x, r.max.y - k)),
		(Pos2::new(r.max.x, r.min.y + k), Pos2::new(r.max.x, r.max.y - k)),
	];
	for (a, b) in sides {
		p.extend(egui::Shape::dashed_line(&[a, b], s, 8.0, 6.0));
	}
	for (cx, cy, a0) in [(r.min.x + k, r.min.y + k, 180.0f32), (r.max.x - k, r.min.y + k, 270.0), (r.max.x - k, r.max.y - k, 0.0), (r.min.x + k, r.max.y - k, 90.0)] {
		let pts: Vec<Pos2> = (0..=8).map(|i| {
			let a = (a0 + i as f32 * 90.0 / 8.0).to_radians();
			Pos2::new(cx + k * a.cos(), cy + k * a.sin())
		}).collect();
		p.add(egui::Shape::line(pts, s));
	}
}

fn kai_icon() -> egui::IconData {
	match image::load_from_memory(include_bytes!("icons/calvard_tool.png")) {
		Ok(im) => {
			let im = im.to_rgba8();
			egui::IconData { width: im.width(), height: im.height(), rgba: im.into_raw() }
		}
		Err(_) => window_icon(),
	}
}

pub fn run() {
	let opts = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_title(TITLE)
			.with_inner_size([1000.0, 740.0])
			.with_min_inner_size([900.0, 640.0])
			.with_drag_and_drop(true)
			.with_icon(kai_icon()),
		centered: true,
		..Default::default()
	};
	if let Err(e) = eframe::run_native(TITLE, opts, Box::new(|cc| Ok(Box::new(App::new(cc))))) {
		eprintln!("{e}");
	}
}
