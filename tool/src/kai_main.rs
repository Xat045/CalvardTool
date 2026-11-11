#![cfg_attr(windows, windows_subsystem = "windows")]
#[allow(unused_imports)]
use calvard_tool::{leprintln, lformat, lprintln};
#[allow(unused_imports)]
use calvard_tool::lang::tr;

use calvard_tool::kai::jobs::{self, Options};
use std::path::PathBuf;

mod gui_common;
mod kai_gui;
mod kai_pick;

struct Console;
impl calvard_tool::jobs::Sink for Console {
	fn log(&self, s: &str) { println!("{s}") }
	fn progress(&self, _d: usize, _t: usize) {}
}

fn main() {
	let args: Vec<String> = std::env::args().skip(1).collect();
	if args.is_empty() && (cfg!(windows) || std::env::var("KISEKI_GUI").is_ok()) {
		kai_gui::run();
		return;
	}
	#[cfg(windows)]
	{
		unsafe {
			windows_sys::Win32::System::Console::AttachConsole(windows_sys::Win32::System::Console::ATTACH_PARENT_PROCESS);
		}
	}
	let mut o = Options {
		game_dir: PathBuf::new(),
		sources: Vec::new(),
		rebuild: false,
		proj_dir: PathBuf::new(),
		out_dir: PathBuf::new(),
		mod_name: "KAI_MOD".into(),
		use_mods: true,
		scena: true,
		tables: true,
		textures: true,
		tex_dds: false,
		tables_json: false,
		check: true,
		other: false,
		keep_failed: true,
		mask: String::new(),
		only_changed: false,
		overwrite_edited: false,
		game: None,
		include: None,
		build_only: None,
	};
	match args.first().map(|s| s.as_str()) {
		Some("png2dds") if args.len() == 4 => {
			let r = (|| -> Result<(), String> {
				let png = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
				let orig = std::fs::read(&args[2]).map_err(|e| e.to_string())?;
				let (d, _) = calvard_tool::kai::dds::compile(&png, &orig)?;
				std::fs::write(&args[3], d).map_err(|e| e.to_string())
			})();
			if let Err(e) = r { eprintln!("{e}"); std::process::exit(1) }
			return;
		}
		Some("roundtrip") if args.len() >= 2 => {
			if let Some(g) = args.get(2).and_then(|s| calvard_tool::kai::game::Game::from_key(s)) { calvard_tool::kai::game::set(g) }
			fn walk(d: &std::path::Path, out: &mut Vec<PathBuf>) {
				if let Ok(rd) = std::fs::read_dir(d) { for e in rd.flatten() { let p = e.path(); if p.is_dir() { walk(&p, out) } else { out.push(p) } } }
			}
			let mut v = Vec::new();
			walk(std::path::Path::new(&args[1]), &mut v);
			v.sort();
			let (mut same, mut diff, mut fail, mut notext) = (0, 0, 0, 0);
			for f in &v {
				let d = match std::fs::read(f) {
					Ok(d) => d,
					Err(e) => { fail += 1; println!("FAIL {}: {e}", f.display()); continue }
				};
				let ext = f.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
				let r: Result<Option<Vec<u8>>, String> = match ext.as_str() {
					"dat" => calvard_tool::kai::scena::decompile(&d).and_then(|(j, _)| calvard_tool::kai::scena::compile(&j).map(Some)),
					"tbl" => calvard_tool::kai::tbl::decompile(&d, &f.file_name().unwrap().to_string_lossy()).and_then(|(b, n, _)| if n == 0 { Ok(None) } else { calvard_tool::kai::tbl::compile(&b, &d).map(Some) }),
					_ => continue,
				};
				match r {
					Ok(None) => notext += 1,
					Ok(Some(o)) if o == d => same += 1,
					Ok(Some(o)) => { diff += 1; lprintln!("DIFF {} ({} -> {} байт)", "DIFF {} ({} -> {} bytes)", f.display(), d.len(), o.len()); if std::env::var("RT_OUT").is_ok() { let _ = std::fs::write(f.with_extension("rt"), &o); } }
					Err(e) => { fail += 1; println!("FAIL {}: {}", f.display(), e.lines().next().unwrap_or("")) }
				}
			}
			lprintln!("одинаково {same}, отличается {diff}, ошибок {fail}, без текста {notext}", "same {same}, different {diff}, errors {fail}, no text {notext}");
			return;
		}
		Some("file") if args.len() >= 2 => {
			if let Some(g) = args.get(2).and_then(|s| calvard_tool::kai::game::Game::from_key(s)) {
				calvard_tool::kai::game::set(g)
			}
			let f = PathBuf::from(&args[1]);
			let name = f.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
			let orig = |ext: &str| std::fs::read(f.with_extension(ext)).map_err(|e| lformat!("нужен оригинал {} рядом: {e}", "the original {} is needed next to it: {e}", f.with_extension(ext).display()));
			let r = (|| -> Result<(PathBuf, Vec<u8>), String> {
				let d = std::fs::read(&f).map_err(|e| e.to_string())?;
				if name.ends_with(".dat") {
					Ok((f.with_extension("json"), calvard_tool::kai::scena::decompile(&d)?.0.into_bytes()))
				} else if name.ends_with(".tbl") {
					Ok((f.with_extension("xlsx"), calvard_tool::kai::tbl::decompile(&d, &name)?.0))
				} else if name.ends_with(".dds") {
					Ok((f.with_extension("png"), calvard_tool::kai::dds::decompile(&d)?.png))
				} else if name.ends_with(".json") {
					Ok((f.with_extension("dat"), calvard_tool::kai::scena::compile(&String::from_utf8_lossy(&d))?))
				} else if name.ends_with(".xlsx") {
					Ok((f.with_extension("tbl"), calvard_tool::kai::tbl::compile_checked(&d, &orig("tbl")?)?.0))
				} else if name.ends_with(".png") {
					Ok((f.with_extension("dds"), calvard_tool::kai::dds::compile(&d, &orig("dds")?)?.0))
				} else {
					Err(tr("неизвестный тип файла: нужен .dat, .tbl, .dds, .json, .xlsx или .png", "unknown file type: expected .dat, .tbl, .dds, .json, .xlsx or .png").into())
				}
			})();
			match r.and_then(|(p, d)| {
				let p = if p.exists() && (name.ends_with(".xlsx") || name.ends_with(".png") || name.ends_with(".json")) {
					p.with_file_name(format!("{}_new.{}", p.file_stem().unwrap_or_default().to_string_lossy(), p.extension().unwrap_or_default().to_string_lossy()))
				} else {
					p
				};
				std::fs::write(&p, d).map(|_| p).map_err(|e| e.to_string())
			}) {
				Ok(p) => println!("{}", p.display()),
				Err(e) => {
					leprintln!("ошибка: {e}", "error: {e}");
					std::process::exit(1);
				}
			}
			return;
		}
		Some("pack") if args.len() == 3 => {
			fn walk(d: &std::path::Path, out: &mut Vec<PathBuf>) {
				if let Ok(rd) = std::fs::read_dir(d) { for e in rd.flatten() { let p = e.path(); if p.is_dir() { walk(&p, out) } else { out.push(p) } } }
			}
			let root = PathBuf::from(&args[1]);
			let mut v = Vec::new();
			walk(&root, &mut v);
			let mut files: Vec<(String, Vec<u8>)> = Vec::new();
			for p in &v {
				match std::fs::read(p) {
					Ok(d) => files.push((p.strip_prefix(&root).unwrap_or(p).to_string_lossy().replace('\\', "/"), d)),
					Err(e) => { eprintln!("{}: {e}", p.display()); std::process::exit(1) }
				}
			}
			files.sort_by(|a, b| a.0.cmp(&b.0));
			if let Err(e) = std::fs::write(&args[2], calvard_tool::kai::p3a::write(&files)) {
				eprintln!("{}: {e}", args[2]);
				std::process::exit(1);
			}
			lprintln!("{} файлов", "{} files", files.len());
			return;
		}
		_ => {}
	}
	let Some(cmd) = args.first().cloned() else {
		lprintln!("CalvardTool decompile --game <папка игры> --proj <папка правки> | compile --proj <..> --out <..> [--name MOD] [--types scena,tables,textures,other] [--mask ..] [--no-mods] [--changed] [--overwrite] [--dds — текстуры как есть, в .dds] [--no-check — без проверки перевода] | extract --game <..> --proj <..> --mask chr0000* | file <файл> [daybreak|daybreak2|horizon] — один .dat/.tbl/.dds/.json/.xlsx/.png туда и обратно", "CalvardTool decompile --game <game folder> --proj <project folder> | compile --proj <..> --out <..> [--name MOD] [--types scena,tables,textures,other] [--mask ..] [--no-mods] [--changed] [--overwrite] [--dds - keep textures as .dds] [--no-check - skip the translation check] | extract --game <..> --proj <..> --mask chr0000* | file <file> [daybreak|daybreak2|horizon] - one .dat/.tbl/.dds/.json/.xlsx/.png there and back");
		return;
	};
	let mut i = 1;
	while i < args.len() {
		let a = args[i].clone();
		let mut val = || { i += 1; args.get(i).cloned().unwrap_or_default() };
		match a.as_str() {
			"--game" => {
				let g: PathBuf = val().into();
				if g.is_file() { o.sources.push(g.clone()); o.game_dir = g.parent().map(|p| p.to_path_buf()).unwrap_or_default(); } else { o.game_dir = g; }
			}
			"--rebuild" => o.rebuild = true,
			"--proj" => o.proj_dir = val().into(),
			"--out" => o.out_dir = val().into(),
			"--name" => o.mod_name = val(),
			"--mask" => o.mask = val(),
			"--types" => { let t = val(); o.scena = t.contains("scena"); o.tables = t.contains("table"); o.textures = t.contains("texture"); o.other = t.contains("other"); }
			"--no-mods" => o.use_mods = false,
			"--changed" => o.only_changed = true,
			"--overwrite" => o.overwrite_edited = true,
			"--dds" => o.tex_dds = true,
			"--tables-json" => o.tables_json = true,
			"--no-check" => o.check = false,
			"--title" => { let v = val(); o.game = calvard_tool::kai::game::Game::from_key(&v); if o.game.is_none() { leprintln!("неизвестная игра {v}: daybreak, daybreak2, horizon", "unknown game {v}: daybreak, daybreak2, horizon"); std::process::exit(2) } }
			x => { leprintln!("неизвестный параметр {x}", "unknown parameter {x}"); std::process::exit(2) }
		}
		i += 1;
	}
	let s = match cmd.as_str() {
		"decompile" => jobs::decompile(&o, &Console),
		"compile" => jobs::compile(&o, &Console),
		"extract" => jobs::extract(&o, &Console),
		_ => { leprintln!("неизвестная команда", "unknown command"); std::process::exit(2) }
	};
	std::process::exit(if s.failed > 0 { 1 } else { 0 });
}
