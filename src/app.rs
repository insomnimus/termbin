use clap::{
	crate_version,
	App,
	AppSettings,
	Arg,
};

pub fn new() -> App<'static> {
	let app = App::new("termbin")
		.version(crate_version!())
		.about("Create a termbin from the command line.")
		.setting(AppSettings::UnifiedHelpMessage);

	let clip = Arg::new("clip")
		.short('c')
		.long("clip")
		.about("Copy the url of the created termbin to the clipboard.");

	let remote = Arg::new("remote")
		.short('r')
		.long("remote")
		.default_value("termbin.com:9999")
		.about("The address where the data will be sent to.")
		.required(true);

	let file = Arg::new("file").about("The file to be shared as a termbin.");

	app.arg(clip).arg(remote).arg(file)
}
