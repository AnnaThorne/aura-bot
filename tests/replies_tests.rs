use aura_bot::replies;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn pick_random_gif_returns_fallback() {
    let mut rng = StdRng::seed_from_u64(42);
    let g = replies::pick_random_gif(&mut rng).expect("expected fallback gif");
    assert!(replies::GIFS.contains(&g.as_str()));
}

#[test]
fn pick_gif_from_category_returns_from_category() {
    let mut rng = StdRng::seed_from_u64(7);
    let opt = replies::pick_gif_from_category(replies::GifCategory::Aura, &mut rng);
    assert!(opt.is_some());
    let g = opt.unwrap();
    let expected = replies::GIF_CATEGORIES
        .iter()
        .find(|(c, _)| *c == replies::GifCategory::Aura)
        .unwrap()
        .1;
    assert!(expected.contains(&g.as_str()));
}
