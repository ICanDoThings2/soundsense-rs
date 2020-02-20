use std::{
	env, error::Error, fs::File, io::Write, path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
	let dest_dir = Path::new(
			&env::var("OUT_DIR")?
		)
		.join("index.html");
	
	#[cfg(not(target_os="windows"))]
	let range_css = include_str!("web/range.css");
	#[cfg(all(target_os="windows", not (feature = "edge")))]
	let range_css = include_str!("web/range-windows.css");
	#[cfg(all(target_os="windows", feature = "edge"))]
	let range_css = include_str!("web/range-windows-edge.css");
	
	let index_html = include_str!("web/index.html")
		.replace("{comment_start}"	, "<!--")
		.replace("{comment_end}"	, "-->")
		.replace("{range}"			, range_css)
		.replace("{w3}"				, include_str!("web/w3.css"))
		.replace("{js}"				, include_str!("web/script.js"));
	let mut html_minifier = html_minifier::HTMLMinifier::new();
	html_minifier.digest(index_html)?;

	File::create(dest_dir)?
		.write_all( html_minifier.get_html().as_bytes() )?;
	
	#[cfg(target_os="windows")]
	{
		let mut res = winres::WindowsResource::new();
		res.set_icon("icons/icon.ico");
		res.compile().unwrap();
	}

	Ok(())
}