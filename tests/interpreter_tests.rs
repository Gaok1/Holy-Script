mod common;
use common::*;

// ── Aritmética ───────────────────────────────────────────────────

#[test]
fn arithmetic_basic() {
    let i = run("let there x of atom be 2 plus 3 times 4\namen\n");
    assert_eq!(get_int(&i, "x"), 14); // precedência: 2 + (3 * 4)
}

#[test]
fn arithmetic_remainder() {
    let i = run("let there x of atom be 10 remainder 3\namen\n");
    assert_eq!(get_int(&i, "x"), 1);
}

#[test]
fn arithmetic_negate() {
    let i = run("let there x of atom be negate 7\namen\n");
    assert_eq!(get_int(&i, "x"), -7);
}

#[test]
fn string_concat_with_plus() {
    let i = run(r#"let there s of word be "hello" plus " world"
amen
"#);
    assert_eq!(get_str(&i, "s"), "hello world");
}

#[test]
fn division_by_zero_is_runtime_error() {
    let msg = run_err("let there x of atom be 1 over 0\namen\n");
    assert!(msg.contains("zero"), "got: {msg}");
}

// ── Condicional ──────────────────────────────────────────────────

#[test]
fn whether_executes_true_branch() {
    let i = run(r#"let there r of word be "no"
whether blessed
    r become "yes"
amen
"#);
    assert_eq!(get_str(&i, "r"), "yes");
}

#[test]
fn whether_skips_false_branch() {
    let i = run(r#"let there r of word be "no"
whether forsaken
    r become "yes"
amen
"#);
    assert_eq!(get_str(&i, "r"), "no");
}

#[test]
fn otherwise_so_chain() {
    let i = run(r#"let there x of atom be 2
let there r of word be ""
whether x is 1
    r become "one"
otherwise so x is 2
    r become "two"
otherwise
    r become "other"
amen
"#);
    assert_eq!(get_str(&i, "r"), "two");
}

// ── Litany / Forsake / Ascend ────────────────────────────────────

#[test]
fn litany_counts_to_five() {
    let i = run(r#"let there i of atom be 0
litany for i no greater than 5
    i become i plus 1
amen
"#);
    assert_eq!(get_int(&i, "i"), 6);
}

#[test]
fn forsake_exits_loop() {
    let i = run(r#"let there i of atom be 0
litany for i no greater than 100
    i become i plus 1
    whether i is 5
        forsake
amen
"#);
    assert_eq!(get_int(&i, "i"), 5);
}

#[test]
fn ascend_skips_body() {
    // soma só os ímpares de 1 a 9
    let i = run(r#"let there i of atom be 0
let there sum of atom be 0
litany for i no greater than 9
    i become i plus 1
    whether i remainder 2 is 0
        ascend
    sum become sum plus i
amen
"#);
    assert_eq!(get_int(&i, "sum"), 25); // 1+3+5+7+9
}

// ── Scripture ────────────────────────────────────────────────────

#[test]
fn scripture_manifest_and_field_access() {
    let i = run(r#"scripture Point
    x of atom
    y of atom

let there p of Point be manifest Point praying 3, 4
let there px of atom be x from p
amen
"#);
    assert_eq!(get_int(&i, "px"), 3);
}

// ── Salm ─────────────────────────────────────────────────────────

#[test]
fn salm_add_returns_sum() {
    let i = run(r#"salm add receiving a of atom, b of atom reveals atom
    reveal a plus b

let there r of atom be hail add praying 10, 32
amen
"#);
    assert_eq!(get_int(&i, "r"), 42);
}

#[test]
fn method_salm_accesses_its() {
    let i = run(r#"scripture Box
    value of atom

salm doubled upon Box reveals atom
    reveal value from its times 2

let there b of Box be manifest Box praying 21
let there r of atom be hail doubled upon b
amen
"#);
    assert_eq!(get_int(&i, "r"), 42);
}

// ── Sin / Confess ─────────────────────────────────────────────────

#[test]
fn confess_catches_sin() {
    let i = run(r#"sin Boom
    msg of word

let there caught of word be "no"

confess
    transgress Boom praying "fire!"
answer for Boom as e
    caught become msg from e
amen
"#);
    assert_eq!(get_str(&i, "caught"), "fire!");
}

#[test]
fn absolve_runs_after_sin() {
    let i = run(r#"sin Err

let there done of dogma be forsaken

confess
    transgress Err
answer for Err
    hail proclaim praying "caught"
absolve
    done become blessed
amen
"#);
    assert!(get_bool(&i, "done"));
}

#[test]
fn unhandled_sin_propagates() {
    let msg = run_err(r#"sin A
sin B

confess
    transgress A
answer for B
    hail proclaim praying "b"
amen
"#);
    assert!(msg.contains("A"), "got: {msg}");
}

// ── Covenant / Discern ───────────────────────────────────────────

#[test]
fn discern_matches_variant() {
    let i = run(r#"covenant Color
    Red
    Blue

let there c of Color be Blue
let there r of word be ""

discern c
    as Red
        r become "red"
    as Blue
        r become "blue"
amen
"#);
    assert_eq!(get_str(&i, "r"), "blue");
}

#[test]
fn discern_falls_through_to_otherwise() {
    let i = run(r#"covenant Dir
    North
    South

let there d of Dir be South
let there r of word be ""

discern d
    as North
        r become "north"
    otherwise
        r become "other"
amen
"#);
    assert_eq!(get_str(&i, "r"), "other");
}

#[test]
fn discern_no_match_no_otherwise_is_runtime_error() {
    let msg = run_err(r#"covenant X
    A
    B

let there v of X be B

discern v
    as A
        hail proclaim praying "a"
amen
"#);
    assert!(msg.contains("B") || msg.contains("discern") || msg.contains("variante"), "got: {msg}");
}
