// run-rustfix

#![warn(clippy::collapsible_str_replace)]

fn get_filter() -> &'static str {
    "u"
}

fn main() {
    let misspelled = "hesuo worpd";

    let p = 'p';
    let s = 's';
    let u = 'u';
    let l = "l";

    // LINT CASES
    let _ = misspelled.replace('s', "l").replace('u', "l");

    let _ = misspelled.replace('s', l).replace('u', l);

    let _ = misspelled.replace('s', "l").replace('u', "l").replace('p', "l");

    let _ = misspelled
        .replace('s', "l")
        .replace('u', "l")
        .replace('p', "l")
        .replace('d', "l");

    // FALLBACK CASES
    // If there are consecutive calls to `str::replace` and all or any chars are variables,
    // recommend the fallback `misspelled.replace(&[s, u, p], "l")`
    let _ = misspelled.replace(s, "l").replace('u', "l");

    let _ = misspelled.replace(s, "l").replace('u', "l").replace('p', "l");

    let _ = misspelled.replace(s, "l").replace(u, "l").replace('p', "l");

    let _ = misspelled.replace(s, "l").replace(u, "l").replace(p, "l");

    // NO LINT CASES
    let _ = misspelled.replace('s', "l");

    let _ = misspelled.replace(s, "l");

    // If the consecutive `str::replace` calls have different `to` arguments, do not lint
    let _ = misspelled.replace('s', "l").replace('u', "p");

    let _ = misspelled.replace(&get_filter(), "l");

    let _ = misspelled.replace(&['s', 'u', 'p'], "l");

    let _ = misspelled.replace(&['s', 'u', 'p'], l);

    let _ = misspelled.replace(&['s', 'u'], "l").replace(&['u', 'p'], "l");

    let _ = misspelled.replace('s', "l").replace(&['u', 'p'], "l");

    let _ = misspelled.replace(&['s', 'u'], "l").replace('p', "l");

    let _ = misspelled.replace(&['s', u, 'p'], "l");

    let _ = misspelled.replace(&[s, u, 'p'], "l");

    let _ = misspelled.replace(&[s, u, p], "l");

    let _ = misspelled.replace(&[s, u], "l").replace(&[u, p], "l");
}
