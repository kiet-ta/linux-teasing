use rand::seq::SliceRandom;

pub fn get_message() -> &'static str {
    let messages = vec![
        "Back to Linux? Finally escaping the Windows dumpster.",
        "Windows blinded you for too long, glad you woke up.",
        "Leaving Windows for Linux… pain first, growth later.",
        "From now on, you’ll feel freedom, while Windows keeps you trapped in the past.",
        "Congrats, you just escaped the Blue Screen disaster.",
        "Back on Linux? Enjoy the difference: fast, smooth, no stupid popups.",
        "Windows gives illusions, Linux gives real power.",
        "Finally leaving Windows, no more being led around by crashes.",
        "Windows is a crying child, Linux is a grown-up playing the real game.",
        "You left Windows, congrats on escaping the endless update nightmare.",
    ];

    messages.choose(&mut rand::thread_rng()).unwrap_or(&"Linux remembers.")
}
