# Finding a demo: alphabetical order and search

Files: [`crates/lineup-app/src/app/demo_menu.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/demo_menu.rs)
and [`crates/lineup-app/src/app/library.rs`](https://github.com/naelolaiz/ferrous-lineup/blob/main/crates/lineup-app/src/app/library.rs)

There are more than two hundred demos now, and **File > Demos** listed
them in the order they were written into the library: London Bridge
before Deck the Halls, Tango before Blues. To find one you opened
submenus until it turned up. Now every list in the menu is in
alphabetical order, and a search box at the top finds any demo from a
few letters of its title, its composer, its genre, its key or its time
signature.

![File > Demos > Songs > Classical and film: the genres from Blues to Tango in alphabetical order, and the classical songs from "Air on the G String" to "Wedding March", with "Für Elise" between "Funeral March" and "Hallelujah Chorus".](../images/step-9-demos-sorted.png)

## Trying it

Open **File > Demos**. The genres under **Songs** and **Styles** go
from Blues to Tango, and each genre's songs and styles are in
alphabetical order, as are the **Twelve-tone** and **Micromodes**
pieces.

Without clicking anything, type **tango**. The box above the submenus
has the keyboard as soon as the menu opens, so the letters go straight
in, and the submenus give way to one list: every demo with "tango" in
its title first, then the songs played as a tango or a milonga, then
the tango styles, and last, fainter, the demos whose notes only
mention the tango, such as Candombe and *St. Louis Blues*, which opens
with a tango introduction. On the right, each line says where the demo lives in the
menu, such as **Songs ▸ Tango**.

![The Demos menu with "tango" typed in the search box. Tango, Tango de salón, Tango del telar, Tango nuevo, Tango síncopa, Tango vals and Tangos flamencos come first; then El Choclo, Gallo ciego, Habanera, La Cumparsita, La Paloma, Milonga, Milonga del río, Vals del puerto and Vals in minor; last, fainter, Candombe and St. Louis Blues. On the right of each, "Songs ▸ Tango" or "Styles ▸ Tango".](../images/step-9-demo-search.png)

Press **Enter** to open the first one, or the **down arrow** to walk
down the list and **Enter** on the one you want. A click works too. As
before, the demo opens and plays from the start, with its style and
rhythm selected in the Library.

Some searches to try:

| Type | Finds |
|---|---|
| `choclo` | El Choclo: any word of the title will do |
| `fur elise` | Für Elise: accents and capitals need not be typed |
| `bach` | Bach's pieces, by composer, after Bachata, whose title starts with "bach"; not Offenbach's Can-can |
| `d minor` | the songs in D minor first, then, fainter, the rest that have a word starting with "d" and a "minor" |
| `3/4` | every demo in three-four time |
| `menor 3` | the micromode pieces built on Menor 3 |
| `waltz` | songs played as a waltz, and the waltz styles |

Every word you type has to be found at the start of a word, in any
order: `minor prelude` puts the two minor preludes first, and `bach`
finds Johann Sebastian Bach but not Jacques Offenbach. When the words
are found apart rather than side by side, or only in the notes about a
demo, it comes at the end, fainter, so the matches you most likely
meant stand out:

![The Demos menu with "d minor" typed. First, in full colour, Toccata in D minor and the eight other songs in D minor: Alma llanera, Dies Irae, Dies irae (Mozart), Gallo ciego, Habanera, House of the Rising Sun, Marinera del callejón and When Johnny Comes Marching Home. Below them, fainter, Anitra's Dance, Asturias, Bambuco del páramo and more, which only have a word starting with "d" and a "minor" somewhere.](../images/step-9-demo-search-key.png)

Press **Esc**, or move to another menu, to close it; the box is empty
again the next time.

## How alphabetical is alphabetical?

Computers compare text letter by letter, by the number each letter
has in Unicode. That gives some odd orders: every capital comes before
every small letter ("Zamba" before "alma llanera"), and an accented
letter comes after all the plain ones ("Ábalos" after "Zamba").
Punctuation gets in the way too: whether "Oh! Susanna" comes before or
after "Oh My Darling" would depend on whether "!" has a smaller number
than a space, which nobody should have to know.

So the menu sorts by a **folded** copy of each title: small letters
only, accents taken off, punctuation dropped, a hyphen turned into a
space. "Für Elise" sorts as "fur elise", right after "Funeral March";
"St. James Infirmary" as "st james infirmary"; "Tico-Tico" as "tico
tico". The menu still shows the real title.

Library catalogues go one step further and skip a leading "The", "El"
or "La", filing *El Choclo* under C. The menu doesn't: it sorts by the
title exactly as you read it, so the first letters run from A to Z and
*El Choclo* sits under E. The search makes up for it: `choclo` finds it
at once.

## The code

### Sorting in the menu, not in the library

The demos live in the `SONGS` array in
`crates/lineup-core/src/library/song.rs`, grouped by genre in the order
they were added. Sorting that array by hand would have been possible,
but every new demo would have to be slotted into the right place, and
several changes adding demos at once would all edit the same lines.
Instead the app makes a sorted copy when the menu is first shown, and
the array keeps its order.

The copy is a list of `Entry` values, one per demo:

```rust
struct Entry {
    demo: Demo,
    /// What its button says, such as "El Choclo · Tango".
    label: String,
    /// Where the menu keeps it, such as "Songs ▸ Tango".
    place: String,
    hover: String,
    /// The title, folded (see [`fold`]): what the menu sorts by and a
    /// search looks at first.
    title: String,
    /// Who wrote it, its feel, genre, key and time signature, folded.
    details: String,
    /// The notes about it, folded.
    about: String,
}
```

It is built once and kept for as long as the app runs, in a
**`LazyLock`**: a `static` whose value is worked out the first time
anyone reads it, not when the program starts. A plain `static` must be
known at compile time, and sorting needs code that only runs at run
time.

```rust
static DEMOS: LazyLock<Demos> = LazyLock::new(|| {
    let mut genres = Genre::ALL.to_vec();
    genres.sort_by(|a, b| by_title(a.name(), b.name()));
    let mut entries = all_entries();
    entries.sort_by(|a, b| {
        a.title
            .cmp(&b.title)
            .then_with(|| a.label.cmp(&b.label))
            .then_with(|| a.place.cmp(&b.place))
    });
    Demos { entries, genres }
});
```

`sort_by` takes a function that says whether `a` comes before, after,
or level with `b`, as an `Ordering`. `then_with` breaks a tie: two
demos with the same folded title ("Dies Irae" and "Dies irae (Mozart)"
are not, but a song and a style can share a name) are ordered by their
label, then by their place, so the order never depends on luck.

Each genre's submenu then walks the one sorted list and keeps that
genre's songs, so every submenu is sorted without sorting it again:

```rust
let songs: Vec<&Entry> = demos
    .entries
    .iter()
    .filter(|e| matches!(e.demo, Demo::Song(song) if song.genre == genre))
    .collect();
```

`matches!` is a small macro that turns a `match` pattern into a `bool`:
"is this a `Demo::Song`, and if so, is its genre this one?"

### Folding text

```rust
fn fold(text: &str) -> String {
    let mut folded = String::with_capacity(text.len());
    for c in text.chars().flat_map(char::to_lowercase) {
        match c {
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą' => folded.push('a'),
            // ... one line per letter ...
            c if c.is_alphanumeric() || matches!(c, '#' | '/' | '+') => folded.push(c),
            c if (c.is_whitespace() || matches!(c, '-' | '–' | '—' | '(' | ')' | '·' | '▸' | ':' | ';'))
                && !folded.is_empty()
                && !folded.ends_with(' ') =>
            {
                folded.push(' ');
            }
            _ => {}
        }
    }
    // ...
}
```

`char::to_lowercase` returns an iterator, not a single `char`, because
some capitals become more than one small letter. `flat_map` strings
those little iterators together into one stream of characters.

`#`, `/` and `+` are kept so that keys ("F#"), time signatures ("3/4")
and grouped beats ("2+2+3/8") can be searched for. A space is only
added when the last character was not one already, so "Row, Row, Row"
becomes "row row row" with single spaces, which matters for the next
part: a search looks for the start of a word, and a word starts after
a space.

A complete job would use the Unicode rules for taking accents apart
(the `unicode-normalization` crate does it). A table of the accented
Latin letters is enough for the titles in the library, and adds no
dependency.

### Searching

A search folds what you typed the same way and splits it into words.
Each word has to start a word of the demo's text:

```rust
fn starts_a_word(text: &str, words: &str) -> bool {
    text.match_indices(words)
        .any(|(at, _)| at == 0 || text[..at].ends_with(' '))
}
```

`match_indices` finds every place `words` occurs in `text`, with the
byte where it starts. The match counts if it is at the very start, or
right after a space: "bach" is found in "johann sebastian bach" but
not in "jacques offenbach". (`text[..at]` is a slice of the text up to
that byte; slicing a `&str` in the middle of a letter would panic, but
`match_indices` only gives places where a match starts, which are
always between letters.)

Then each demo gets a **rank**, or none if one of the words is nowhere
to be found:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    TitleStart,
    TitleWords,
    Details,
    Words,
    About,
}
```

`TitleStart`: the title starts with what you typed. `TitleWords`:
every word you typed starts a word of the title. `Details`: what you
typed, as typed, is in who wrote it, the feel, genre, key or time
signature, so "d minor" in a song whose key is "D minor". `Words`: the
words were all found in those, but apart. `About`: some word was only
found in the notes about the demo.

The key needed a little musical care. Many minor songs are written in
the **harmonic minor** scale, whose seventh note is raised to give a
strong pull back to the tonic. A song in D harmonic minor is still in
the key of D minor, so its searchable details say both, "D harmonic
minor" and "D minor", and `d minor` finds the Dies irae of Mozart's
Requiem as well as Toccata in D minor.

Deriving `Ord` on an enum orders its variants as they are written, so
`Rank::TitleStart < Rank::About`, and sorting the matches by rank puts
the best first:

```rust
// A stable sort, so equal ranks stay in alphabetical order.
found.sort_by_key(|&(_, rank)| rank);
```

The demos were already alphabetical, and Rust's `sort_by_key` is
**stable**: things that compare equal keep the order they had. So
within each rank the matches stay alphabetical without saying so.
(`sort_unstable_by_key` is a little faster but makes no such promise.)

The rank decides the colour too: from `Rank::Words` on, the label is
drawn with `RichText::weak()`, fainter than the rest. The comparison
`rank >= Rank::Words` works for the same reason the sort does.

### A text box in a menu

Three details of egui, the GUI library, needed care:

- **A click inside must not close the menu.** egui menus close on any
  click, inside or outside, so that clicking an item closes them. The
  Demos submenu is made with `SubMenuButton` and a `MenuConfig` whose
  `close_behavior` is `PopupCloseBehavior::CloseOnClickOutside`.
  Clicking a demo still closes the whole File menu, because the code
  calls `ui.close()` when one is picked.
- **The box takes the keyboard when the menu opens.** egui calls the
  menu's code every frame it is shown, and gives each pass a number
  (`ctx.cumulative_pass_nr()`). The menu stores the number of the last
  pass it was shown in; if that was not the pass just before this one,
  the menu has just opened, so the box is emptied and
  `request_focus()` is called on it.
- **The down arrow leaves the box.** egui menus already move between
  their items with the arrow keys, but a text box keeps the arrow keys
  for itself. So before the box is drawn, while it has focus, the menu
  takes the down arrow out of the input with `consume_key`, and gives
  the focus to the first match. From there the menu's own arrow keys
  take over.

What you typed is kept between frames in egui's own memory
(`ctx.data_mut(|data| data.insert_temp(id, query))`), keyed by an `Id`,
instead of a new field in `LineupApp`: it only matters while the menu
is open.

## Tests

`demo_menu.rs` checks that folding takes off accents and punctuation,
that every genre's songs and the genres themselves come out in
alphabetical order, that a search finds titles before composers and
needs every word, that it finds songs by composer (Bach but not
Offenbach), genre, key (every song in D minor, ahead of Hungarian
Dance No. 5, which only has a "Dance" and is in G minor) and time
signature, and that weaker matches come last. Run them with:

```sh
cargo test -p lineup-app demo_menu
```

## Try it yourself

1. Search for your favourite composer. Which demos come up fainter, and
   why?
2. In `fold`, remove `'/'` from the characters that are kept. Which
   test fails, and what does a search for `3/4` find now?
3. Library catalogues skip a leading article when sorting. Write a
   function `sort_key(title: &str) -> String` that folds the title and
   then drops a first word "the", "a", "an", "el", "la", "los" or
   "las". Give `Entry` a `sort_key` field filled with it, and sort
   `DEMOS.entries` by that instead of `title` (keep `title` for the
   search, so `el ch` still finds *El Choclo*). Where does *El Choclo*
   go in the Tango submenu now? Do you like it better?
