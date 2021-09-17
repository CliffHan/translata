mod google;
mod translator;

use arboard::Clipboard;
use failure::Fallible;
use google::GoogleTranslator;
use headless_chrome::{Browser, LaunchOptions};
use std::sync::mpsc::channel;
use std::time::Duration;
use translator::Translator;
use clap::{crate_version, crate_authors, crate_description, AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(version = crate_version!(), author = crate_authors!(), about = crate_description!())]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Web translator to be used
    #[clap(short = 'u', long = "use", default_value = "google")]
    translator: String,
    /// Language for text to be translated from
    #[clap(short, long, default_value = "auto")]
    from: String,
    /// Language for text to be translated to
    #[clap(short, long, default_value = "zh-CN")]
    to: String,
    /// Browser will be seen when set
    #[clap(short, long)]
    browser: bool,
}

fn main() -> Fallible<()> {
    env_logger::init();

    // Open translate site
    let opts: Opts = Opts::parse();
    println!("Starting translator `{}`, from=`{}`, to=`{}`, visible=`{}`.", opts.translator, opts.from, opts.to, opts.browser);
    let options = LaunchOptions::default_builder()
        .idle_browser_timeout(Duration::MAX)
        .headless(!opts.browser)
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = Browser::new(options)?;
    let tab = browser.wait_for_initial_tab()?;
    println!("Opening translator: GoogleTranslator");
    tab.navigate_to(GoogleTranslator::get_url())?
        .wait_until_navigated()?;

    let google_translator = GoogleTranslator::new(tab);
    println!("Now setting language.");
    google_translator.set_language(Some(&opts.from), Some(&opts.to))?;
    // let result = google_translator.translate("Are you ok?\n\nI'm fine.".into());
    // log::debug!("Result = {:?}", result);
    // let result = google_translator.translate("So many things did I recall. Say we were young, just flying high.".into());
    // log::debug!("Result = {:?}", result);

    let (tx, rx) = channel::<Option<String>>();
    let tx2 = tx.clone();
    std::thread::spawn(move || {
        println!("Register hotkey Ctrl+T for translate.");
        let mut hk = hotkey::Listener::new();
        hk.register_hotkey(hotkey::modifiers::CONTROL, 'Q' as u32, move || {
            tx.send(None).unwrap();
        })
        .unwrap();
        hk.register_hotkey(hotkey::modifiers::CONTROL, 'T' as u32, move || {
            let mut clipboard = Clipboard::new().unwrap();
            let text = clipboard.get_text().unwrap();
            // println!("Clipboard text was: {}", text);
            tx2.send(Some(text)).unwrap();
        })
        .unwrap();
        println!("Ready to listen! Press Ctrl-C or Ctrl-Q to quit");
        hk.listen();
    });
    loop {
        let text_option = rx.recv().unwrap();
        if let Some(text) = text_option {
            println!("Got text from clipboard, translating...");
            let result = google_translator.translate(text);
            // log::debug!("Result = {:?}", result);
            if let Ok(translated) = result {
                let mut clipboard = Clipboard::new().unwrap();
                clipboard.set_text(translated).unwrap();
                println!("Translate finished, result set to clipboard");
            } else {
                println!("Translate failed, result={:?}", result);
            }
        } else {
            println!("Bye Bye");
            break;
        }
    }

    Ok(())
}
