use proc_macro::{Delimiter, Span, TokenStream, TokenTree};
use quote::quote;

/// Render compile-time SHX markup into a String using Rust formatting.
#[proc_macro]
pub fn render(input: TokenStream) -> TokenStream {
    expand(input, false).unwrap_or_else(|error| error.into_compile_error().into())
}

/// Print compile-time SHX markup without an intermediate String or newline.
#[proc_macro]
pub fn print(input: TokenStream) -> TokenStream {
    expand(input, true).unwrap_or_else(|error| error.into_compile_error().into())
}

fn expand(input: TokenStream, print: bool) -> syn::Result<TokenStream> {
    let mut tokens = input.into_iter();
    let first = tokens.next().ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected braced markup or a string literal",
        )
    })?;
    let span = proc_macro2::Span::from(first.span());
    let source = match first {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
            let mut text = String::new();
            markup(group.stream(), &mut text, &mut None);
            text.trim().to_owned()
        }
        TokenTree::Literal(literal) => syn::parse_str::<syn::LitStr>(&literal.to_string())?.value(),
        _ => {
            return Err(syn::Error::new(
                span,
                "expected braced markup or a string literal",
            ));
        }
    };
    let rest: TokenStream = tokens.collect();
    if let Some(token) = rest.clone().into_iter().next()
        && !matches!(token, TokenTree::Punct(ref punct) if punct.as_char() == ',')
    {
        return Err(syn::Error::new(
            token.span().into(),
            "expected a comma before format arguments",
        ));
    }
    let arguments = proc_macro2::TokenStream::from(rest);
    let (colored, plain) = compile(&source);
    let colored = syn::LitStr::new(&colored, span);
    let plain = syn::LitStr::new(&plain, span);
    let operation = if print {
        quote!(::std::print)
    } else {
        quote!(::std::format)
    };
    Ok(quote!({
        if ::std::env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty()) {
            #operation!(#plain #arguments)
        } else {
            #operation!(#colored #arguments)
        }
    })
    .into())
}

// Reconstruct from the actual tokens, rather than a group's source_text: groups
// forwarded by another macro may have source text unrelated to their contents.
// Native stable spans preserve whitespace without TokenStream::to_string's
// inserted spaces around punctuation and tags.
fn markup(stream: TokenStream, text: &mut String, previous: &mut Option<Span>) {
    for token in stream {
        if let TokenTree::Group(group) = token {
            if group.delimiter() == Delimiter::Brace {
                let inner: Vec<_> = group.stream().into_iter().collect();
                if let [TokenTree::Group(escaped)] = inner.as_slice()
                    && escaped.delimiter() == Delimiter::Brace
                {
                    let mut literal = String::new();
                    markup(escaped.stream(), &mut literal, &mut None);
                    append(&format!("{{{{{literal}}}}}"), group.span(), text, previous);
                    continue;
                }
                // Rustfmt may lay out format fields like Rust blocks. Their
                // internal token spacing is not part of the rendered text.
                let field: String = group
                    .stream()
                    .to_string()
                    .chars()
                    .filter(|ch| !ch.is_whitespace())
                    .collect();
                append(&format!("{{{field}}}"), group.span(), text, previous);
                continue;
            }
            let delimiters = match group.delimiter() {
                Delimiter::Parenthesis => Some(("(", ")")),
                Delimiter::Brace => Some(("{", "}")),
                Delimiter::Bracket => Some(("[", "]")),
                Delimiter::None => None,
            };
            if let Some((open, _)) = delimiters {
                append(open, group.span_open(), text, previous);
            }
            markup(group.stream(), text, previous);
            if let Some((_, close)) = delimiters {
                append(close, group.span_close(), text, previous);
            }
        } else {
            append(&token.to_string(), token.span(), text, previous);
        }
    }
}

fn append(value: &str, span: Span, text: &mut String, previous: &mut Option<Span>) {
    if let Some(last) = previous {
        let end = last.end();
        let start = span.start();
        if start.line() > end.line() {
            text.push(' ');
        } else if start.line() == end.line() && start.column() > end.column() {
            text.extend(std::iter::repeat_n(' ', start.column() - end.column()));
        }
    }
    text.push_str(value);
    *previous = Some(span);
}

fn compile(source: &str) -> (String, String) {
    // Hide Rust format fields from the markup parser, particularly alignment
    // specifiers such as {name:<10} that contain angle brackets.
    let mut marker = "\0SHX".to_owned();
    while source.contains(&marker) {
        marker.push('_');
    }
    let mut protected = String::new();
    let mut fields = Vec::new();
    let mut rest = source;
    while let Some(start) = rest.find('{') {
        protected.push_str(&rest[..start]);
        let field = &rest[start..];
        let end = if field.starts_with("{{") {
            2
        } else {
            field.find('}').map_or(field.len(), |end| end + 1)
        };
        protected.push_str(&format!("{marker}{}\0", fields.len()));
        fields.push(&field[..end]);
        rest = &field[end..];
    }
    protected.push_str(rest);
    let document = shellx_core::parser::parse(&protected);
    let mut colored = Vec::new();
    shellx_core::render::render(&document, &mut colored).expect("writing to a Vec cannot fail");
    let mut colored = String::from_utf8(colored).expect("renderer preserves UTF-8");
    let mut plain: String = document
        .nodes
        .iter()
        .filter_map(|node| match node {
            shellx_core::ShxNode::Text(text) => Some(*text),
            _ => None,
        })
        .collect();
    for (index, field) in fields.iter().enumerate() {
        let key = format!("{marker}{index}\0");
        colored = colored.replace(&key, field);
        plain = plain.replace(&key, field);
    }
    (colored, plain)
}
