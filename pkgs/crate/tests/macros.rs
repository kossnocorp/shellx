use shellx::render;

fn expected(colored: &str, plain: &str) -> String {
    if std::env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty()) {
        plain.to_owned()
    } else {
        colored.to_owned()
    }
}

#[test]
fn braced_markup_preserves_spacing_and_inherits_styles() {
    assert_eq!(
        render!({
            <green bold>Hello, <bold>{name}!</bold></green>
        }, name = "Sasha"),
        expected("\x1b[1;32mHello, Sasha!\x1b[0m", "Hello, Sasha!")
    );
    assert_eq!(render!({Hello,  world!}), "Hello,  world!");
    assert_eq!(render!({世界: {name}!}, name = "你好"), "世界: 你好!");
}

#[test]
fn supports_rust_formatting_and_literal_braces() {
    let name = "Sasha";
    assert_eq!(
        render!({<green>{name}</green>}),
        expected("\x1b[32mSasha\x1b[0m", name)
    );
    assert_eq!(
        render!(
            {
                {
                    1
                }
                {}
                { { name } }
            },
            "a",
            "b"
        ),
        "b a {name}"
    );
    assert_eq!(
        render!("<red>{name:<8}</red> {0:04} {{}}\n", 42),
        expected("\x1b[31mSasha   \x1b[0m 0042 {}\n", "Sasha    0042 {}\n")
    );
    assert_eq!(render!({{name:>8}}), "   Sasha");
    assert_eq!(render!({<span>{{hello world}}</span>}), "{hello world}");
    assert_eq!(render!({<span>{{{name}}}</span>}), "{Sasha}");
    assert_eq!(
        render!("{value:.precision$}", value = 1.2345, precision = 2),
        "1.23"
    );
}

#[test]
fn attributes_restore_parent_styles() {
    assert_eq!(
        render!({<red bold>a<span bold=false bg=blue>b</span>c</red>}),
        expected("\x1b[1;31ma\x1b[0;31;44mb\x1b[0;1;31mc\x1b[0m", "abc")
    );
}

#[test]
fn values_are_borrowed_evaluated_once_and_never_parsed() {
    let mut calls = 0;
    let value = String::from("<red>{name}</red>");
    let rendered = render!({{0}/{0}}, { calls += 1; &value });
    assert_eq!(calls, 1);
    assert_eq!(rendered, "<red>{name}</red>/<red>{name}</red>");
    assert_eq!(value, "<red>{name}</red>");
    assert_eq!(
        render!("<bold>{}</bold>", ""),
        expected("\x1b[1m\x1b[0m", "")
    );
}

#[test]
fn works_through_macro_rules() {
    macro_rules! greeting {
        ($name:expr) => { render!({<green>Hello, {name}!</green>}, name = $name) };
    }
    assert_eq!(
        greeting!("Sasha"),
        expected("\x1b[32mHello, Sasha!\x1b[0m", "Hello, Sasha!")
    );
    assert_eq!(render!(""), "");
}

#[test]
fn print_writes_without_a_newline_in_both_color_modes() {
    if std::env::var_os("SHELLX_MACRO_PRINT_CHILD").is_some() {
        std::print!("OUTPUT_START");
        let result: () = shellx::print!({<green>{name}</green>}, name = "Sasha");
        assert_eq!(result, ());
        std::print!("OUTPUT_END");
        return;
    }
    for (no_color, text) in [("", "\x1b[32mSasha\x1b[0m"), ("1", "Sasha")] {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "print_writes_without_a_newline_in_both_color_modes",
                "--nocapture",
            ])
            .env("SHELLX_MACRO_PRINT_CHILD", "1")
            .env("NO_COLOR", no_color)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(
            stdout.contains(&format!("OUTPUT_START{text}OUTPUT_END")),
            "{stdout:?}"
        );
    }
}
