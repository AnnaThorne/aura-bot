use rand::seq::SliceRandom;
use rand::Rng;

/// Strongly-typed GIF categories
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GifCategory {
    Beam,
    Aura,
}

/// Piccolo-themed quotes and optional associated GIF category.
/// Update these tuples to change quote text or category mapping.
pub static QUOTE_CATEGORIES: &[(&str, Option<GifCategory>)] = &[
    ("They must be using the pull up techhnique.", None),
    ("SPECIAL BEAM!!", Some(GifCategory::Beam)),
    ("They're using the sun-kissed technique!", None),
    ("What is life without aura farming?", None),
    ("They're definitely using that technique.", None),
    (
        "The pull-up technique. It's a technique\nwhere you arrive late on purpose with a grand entrance.",
        None,
    ),
    ("That guy aura farms for a living tho.",None),
    ("There's an aura farmer close by.", None),
    (
        "The King Charles technique - It's an aura farming technique where you\nmake people bow down when you\nwalk past or by them.",
        None,
    ),
    ("Yes, yes, just making sure you peeped the fit.", Some(GifCategory::Aura)),
    (
        "The explosion technique. A technique\nwhereby you'd rather stand still during an\nexplosion than escape from it. I've used\nit quite a bit over the years.",
        None,
    ),
    (
        "Pardon me. I find it irksome to be looked down upon by someone smaller.\nI'm sure you can appreciate my dilemma.",
        None,
    ),
    ("This dude because of his mass, his\ngirth, his power.", None),
    ("You seem to also be familiar with the\nsun-kissed technique.", Some(GifCategory::Aura)),
    ("Who decided that?", None),
    ("Ah, the pull-up technique. A classic.", None),
    ("Ain't I clean though?", Some(GifCategory::Aura)),
    ("Yeah, I'm familiar with the\ntechnique. The nonchalant technique.", None),
    ("The game is truly back.", None),
    ("You see how they're cape farming ine explosion?", None),
    ("You all right?", None),
    ("It's just a trick I picked up from\nancient Tibetan monks.", None),
    ("What? They're using my technique against me?", None),
    ("Another trick I learned from the Tibetan monks.", None),
    ("The clone technique. I should have seen this coming.", None),
    (
        "This guy is legendary? Based on what?\n What has he done? How long has he been in THE GAME?",
        None,
    ),
];

/// Categories of GIFs - each category contains a small curated list of GIF URLs.
pub static GIF_CATEGORIES: &[(GifCategory, &[&str])] = &[
    (
        GifCategory::Beam,
        &[
            "https://tenor.com/view/piccolo-special-beam-cannon-dragon-ball-namekian-dragonball-gif-1771171787201928933",
        ],
    ),
    (
        GifCategory::Aura,
        &[
            "https://tenor.com/view/piccolo-piccolo-aura-piccolo-aura-farming-cape-piccolo-we-see-the-fit-gif-12978094043841215022",
        ],
    ),
];

/// Generic GIFs used as a fallback when a category or specific GIF is not available.
pub static GIFS: &[&str] = &[
    "https://tenor.com/view/piccolo-dbz-dragon-ball-piccolo-standing-on-tower-tower-gif-13558229454457057953",
    "https://tenor.com/view/piccolo-piccolo-aura-piccolo-aura-farming-cape-piccolo-we-see-the-fit-gif-12978094043841215022",
];

/// Choose a random quote (returns the text and optional category)
pub fn pick_quote<'a, R: Rng + ?Sized>(rng: &mut R) -> (&'static str, Option<GifCategory>) {
    let &(q, cat) = QUOTE_CATEGORIES.choose(rng).unwrap();
    (q, cat)
}

/// Pick a gif URL from a category, if any
pub fn pick_gif_from_category<R: Rng + ?Sized>(cat: GifCategory, rng: &mut R) -> Option<String> {
    GIF_CATEGORIES
        .iter()
        .find(|(c, _)| *c == cat)
        .and_then(|(_, gifs)| gifs.choose(rng).map(|s| s.to_string()))
}

/// Pick a random gif from fallback list
pub fn pick_random_gif<R: Rng + ?Sized>(rng: &mut R) -> Option<String> {
    GIFS.choose(rng).map(|s| s.to_string())
}
