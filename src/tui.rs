use crate::from_u64;
use cursive::{
    Cursive,
    event::*,
    theme::Theme,
    views::*,
    view::*,
};
use fst::*;
use fst_regex::Regex;
use crate::store::Store;

pub fn theme() -> Theme {
    let mut theme = Theme::default();
    theme.shadow = false;
    theme
}

/// Initialize cursive, taking control of stdin and stdout. This will set the default settings like
/// theme and global callbacks
pub fn initialize_cursive() -> Option<Cursive> {
    let mut siv = Cursive::termion().ok()?;
    siv.set_theme(theme());
    siv.add_global_callback('q', |s| s.quit());

    Some(siv)
}

pub fn character_search<I>(results: I) -> impl IntoBoxedView
    where I: Iterator<Item=(String, u64)>
{
    let mut list_view = SelectView::new()
        .on_submit(|cursive: &mut Cursive, value: &u64| {
            let c: char = from_u64(*value)
                .expect( "Could not parse character");
            cursive.add_layer(save_prompt(c));
        });
    add_items(&mut list_view, results);
    let list_view = list_view.with_id("select");

    let list_view = OnEventView::new(list_view)
        .on_pre_event_inner('k', |s, _| {
            s.get_mut().select_up(1);
            Some(EventResult::Consumed(None))
        })
        .on_pre_event_inner('j', |s, _| {
            s.get_mut().select_down(1);
            Some(EventResult::Consumed(None))
        })
        .on_event('/', |s| {
            s.focus(&Selector::Id("search"))
                .expect("could not focus search");
        });

    let scroll_view = ScrollView::new(list_view);

    LinearLayout::vertical()
        .child(scroll_view)
        .child(search_view())
}

fn search_view() -> impl View {
    OnEventView::new(
        EditView::new()
            .on_submit(|cursive, s| {
                cursive.call_on_id("select", |v: &mut SelectView<u64>| update_search(v, s));
                cursive.focus(&Selector::Id("select"))
                    .ok();
            })
            .fixed_height(1)
            .min_width(10)
            .with_id("search")
    ).on_event(Key::Esc, |s| {
        s.focus(&Selector::Id("select"))
            .expect("could not focus select");
    } )
}

fn update_search(view: &mut SelectView<u64>, query: &str) {
    view.clear();
    let results = search(query);
    add_items(view, results.into_iter());
}

pub fn search(query: &str) -> Vec<(String, u64)> {
    // Modify the regex
    // Case insensitive, and allows leading and trailing characters
    let regex_string = format!("(?i).*{}.*", query);
    let re = Regex::new(regex_string.as_str())
        .expect("regex compile");

    let unicode_map = crate::mk_map();
    unicode_map
        .search(&re)
        .into_stream()
        .into_str_vec()
        .expect("convert keys to utf-8")
}

fn add_items<I>(view: &mut SelectView<u64>, items: I)
    where I: Iterator<Item=(String, u64)>
{
    for (description, v) in items {
        if let Some(character) = from_u64(v) {
            let line = format!("{} = {:04X}, {}", character, v, description);
            view.add_item(line, v);
        } else {
            log::warn!("Index number {} could not be decoded to a character", v);
        }
    }
}

pub fn save_prompt(val_to_save: char) -> impl IntoBoxedView {
    OnEventView::new(
        Dialog::new()
            .title("Save to")
            .padding((1, 1, 1, 0))
            .content(
                EditView::new()
                    .filler(" ")
                    .on_submit(move |cursive, var_name| save(cursive, var_name, val_to_save))
                    .fixed_width(20),
            )
            .button("Ok", move |s| {
                let name = s.call_on_id(
                    "name",
                    |view: &mut EditView| view.get_content(),
                ).unwrap();
                save(s, &name, val_to_save);
            })
    ).on_event(Key::Esc, |s| {s.pop_layer();} )
}

fn save(s: &mut Cursive, var_name: &str, value: char) {
    if var_name.is_empty() {
        s.add_layer(Dialog::info("Enter a var name"));
    } else {
        let mut store = Store::load_file()
            .expect("Error loading Store file.");
        store.saved.insert(var_name.to_string(), value);
        store.save_file()
            .expect("Error saving Store file.");

        s.quit();
    }
}
