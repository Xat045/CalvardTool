#![cfg_attr(windows, windows_subsystem = "windows")]
#[allow(unused_imports)]
use calvard_tool::{leprintln, lformat, lprintln};
#[allow(unused_imports)]
use calvard_tool::lang::tr;

mod gui_common;
mod compare_gui;
mod kai_buttons;

fn main() {
	let args: Vec<String> = std::env::args().skip(1).collect();
	if args.is_empty() && (cfg!(windows) || std::env::var("KISEKI_GUI").is_ok()) {
		compare_gui::run_main();
		return;
	}
	#[cfg(windows)]
	{
		unsafe {
			windows_sys::Win32::System::Console::AttachConsole(windows_sys::Win32::System::Console::ATTACH_PARENT_PROCESS);
		}
	}
	if args.len() < 3 {
		lprintln!("CalvardCompare <английская папка> <русская папка> <поиск>", "CalvardCompare <English folder> <Russian folder> <search>");
		return;
	}
	use calvard_tool::kai::compare;
	if let Some(g) = std::env::var("ED7").ok().filter(|g| !g.is_empty()) {
		compare::set_ed7(Some(if g == "azure" { calvard_tool::scena::Title::Azure } else { calvard_tool::scena::Title::Zero }));
	}
	compare::EXACT.store(std::env::var("WHOLE").is_ok(), std::sync::atomic::Ordering::Relaxed);
	let t = std::time::Instant::now();
	let cache = std::env::var("KC_CACHE").ok().map(std::path::PathBuf::from);
	let d = compare::load_cached(args[0].as_ref(), args[1].as_ref(), &|_, _| {}, cache.as_deref());
	let both = d.pairs.iter().filter(|p| p.en.is_some() && p.ru.is_some()).count();
	let only_en = d.pairs.iter().filter(|p| p.ru.is_none()).count();
	let only_ru = d.pairs.iter().filter(|p| p.en.is_none()).count();
	lprintln!("файлов {}, строк {} (пары {both}, только англ {only_en}, только рус {only_ru}), {:.1} с", "{} files, {} lines (pairs {both}, English only {only_en}, Russian only {only_ru}), {:.1} s", d.files.len(), d.pairs.len(), t.elapsed().as_secs_f32());
	for w in &d.warnings { println!("⚠ {w}") }
	if let Some(w) = &d.game_warning { lprintln!("⚠ файлы из разных игр: {w}", "⚠ files from different games: {w}") }
	lprintln!("служебных имён («—НЕ ТРОГАТЬ—»): {}", "service names ('DO NOT TOUCH'): {}", d.pairs.iter().filter(|p| p.service).count());
	if std::env::var("FILESTATS").is_ok() {
		let mut m: std::collections::BTreeMap<String, (usize, usize, usize)> = Default::default();
		for p in &d.pairs {
			if std::env::var("FILESTATS").ok().as_deref() == Some(p.file.as_str()) && p.en.is_none() { if let Some(r) = &p.ru { println!("ONLYRU {} {} {}", p.id, r.path.display(), r.text.chars().take(50).collect::<String>()) } }
			let e = m.entry(p.file.clone()).or_default();
			match (&p.en, &p.ru) { (Some(_), Some(_)) => e.0 += 1, (Some(_), None) => e.1 += 1, (None, Some(_)) => e.2 += 1, _ => {} }
		}
		for (f, (b, oe, or)) in m { if oe + or > 0 { lprintln!("{f}\tпары {b}\tтолько англ {oe}\tтолько рус {or}", "{f}\tpairs {b}\tEnglish only {oe}\tRussian only {or}") } }
		return;
	}
	if std::env::var("CHECKS").is_ok() {
		let r = calvard_tool::kai::checks::run(&d);
		let mut by: std::collections::BTreeMap<(usize, String), usize> = Default::default();
		for f in &r.findings { *by.entry((f.cat, f.what.clone())).or_default() += 1; }
		lprintln!("проверено {}, находок {}", "checked {}, findings {}", r.checked, r.findings.len());
		for (c, (t, _, _)) in calvard_tool::kai::checks::cats().iter().enumerate() {
			lprintln!("  раздел {c} «{t}»: {}", "  section {c} '{t}': {}", r.findings.iter().filter(|f| f.cat == c).count());
		}
		let mut v: Vec<_> = by.into_iter().collect();
		v.sort_by(|a, b| b.1.cmp(&a.1));
		for ((c, w), n) in v.iter().take(40) { println!("{c}\t{n}\t{w}") }
		let lim: usize = std::env::var("CHECKS").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
		let cat: usize = std::env::var("CHECKS_CAT").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
		for f in r.findings.iter().filter(|f| f.cat == cat && f.pair != usize::MAX).take(lim) {
			let p = &d.pairs[f.pair];
			println!("[{} {}] {}\n  EN {}\n  RU {}", p.file, p.id, f.what, p.en.as_ref().map(|s| s.text.as_str()).unwrap_or(""), p.ru.as_ref().map(|s| s.text.as_str()).unwrap_or(""));
		}
		return;
	}
	if std::env::var("DUMPPAIRS").is_ok() {
		for p in &d.pairs {
			let (Some(a), Some(b)) = (&p.en, &p.ru) else { continue };
			println!("{}\t{}\t{}\t{}", p.file, p.id, a.text.replace('\n', "\\n").replace('\t', " "), b.text.replace('\n', "\\n").replace('\t', " "));
		}
		return;
	}
	let (hits, total) = compare::search(&d, &args[2], 0, std::env::var("WHOLE").is_ok(), 20);
	lprintln!("найдено {total}", "found {total}");
	let dict_all = std::env::var("DICT").ok().map(|p| compare_gui::read_dict(p.as_ref()).unwrap_or_else(|e| { eprintln!("{e}"); std::process::exit(1) })).unwrap_or_default();
	let (all, _) = compare::search(&d, &args[2], 0, std::env::var("WHOLE").is_ok(), usize::MAX);
	for w in compare::ru_words_for_query(&dict_all, &args[2]) {
		let (t, l) = compare::count_ru(&d, &all, &w);
		lprintln!("  «{w}» в русском: {t} раз в {l} строках", "  '{w}' in Russian: {t} times in {l} lines");
	}
	let (den, dict) = compare::terms_for_query(&dict_all, &args[2]);
	lprintln!("подсветка: англ {:?}, рус {:?}", "highlights: English {:?}, Russian {:?}", den, dict);
	for i in hits {
		let p = &d.pairs[i];
		println!("[{} {}]\n  EN: {}\n  RU: {}", p.file, p.id,
			p.en.as_ref().map(|s| compare::clean(&s.text, false)).unwrap_or("—".into()),
			p.ru.as_ref().map(|s| { let t = compare::clean(&s.text, false); let h = compare::highlights(&t, &dict); let c: Vec<char> = t.chars().collect(); let mut o = String::new(); let mut k = 0; for (a, b) in h { o.extend(&c[k..a]); o.push('['); o.extend(&c[a..b]); o.push(']'); k = b; } o.extend(&c[k..]); o }).unwrap_or("—".into()));
	}
}
