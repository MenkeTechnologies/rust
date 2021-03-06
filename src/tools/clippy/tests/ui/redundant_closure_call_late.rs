// non rustfixable, see redundant_closure_call_fixable.rs

#![warn(clippy::redundant_closure_call)]

fn main() {
    let mut i = 1;

    // don't lint here, the closure is used more than once
    let closure = |i| i + 1;
    i = closure(3);
    i = closure(4);

    // lint here
    let redun_closure = || 1;
    i = redun_closure();

    // shadowed closures are supported, lint here
    let shadowed_closure = || 1;
    i = shadowed_closure();
    let shadowed_closure = || 2;
    i = shadowed_closure();

    // don't lint here
    let shadowed_closure = || 2;
    i = shadowed_closure();
    i = shadowed_closure();
}
