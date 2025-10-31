use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Napaka" => "Err",
        "Uspešno" => "Ok",
        "Niz" => "String",
        "Slovar" => "HashMap",
        "Privzeto" => "Default",
        "Okvara" => "Error",
        "Mogoče" => "Option",
        "Nekaj" => "Some",
        "Nič" => "None",
        "Izid" => "Result",
        "Jaz" => "Self",
        "tiskajvr" => "println",
        "prekini" => "break",
        "asinhrona" => "async",
        "dočakaj" => "await",
        "zanka" => "loop",
        "premakni" => "move",
        "zaboj" => "crate",
        "nedosegljiva_koda" => "unreachable_code",
        "kot" => "as",
        "stalnica" => "const",
        "značilnost" => "trait",
        "nevarno" => "unsafe",
        "znotraj" => "in",
        "iz" => "from",
        "dinamičen" => "dyn",
        "odmotaj" => "unwrap",
        "privzeto" => "default",
        "kot_sklic" => "as_ref",
        "vi" => "io",
        "zunanji" => "extern",
        "neresnično" => "false",
        "funkcija" => "fn",
        "odlično" => "super",
        "vstavi" => "insert",
        "pridobi" => "get",
        "dovoli" => "allow",
        "jojmene" | "porkamadona" | "čuj" => "panic",
        "modul" => "mod",
        "spremenljiv" => "mut",
        "nov" => "new",
        "kjer" => "where",
        "za" => "for",
        "pridobi_ali_vstavi_z" => "get_or_insert_with",
        "glavna" => "main",
        "javna" => "pub",
        "kaj" => None?,
        "vrni" => "return",
        "izvedba" => "impl",
        "sklic" => "ref",
        "ujemanje" => "match",
        "če" => "if",
        "sicer" => "else",
        "jaz" => "self",
        "bodi" => "let",
        "ustaljen" => "static",
        "skupek" => "struct",
        "pričakuj" => "expect",
        "dokler" => "while",
        "uporabi" => "use",
        "v" => "into",
        "resnično" => "true",
        "naštevanje" => "enum",
        "Skupina" => "Group",
        "Značilka" => "Ident",
        "TokŽetonov" => "TokenStream",
        "DrevoŽetonov" => "TokenTree",
        "v_niz" => "to_string",
        "kot_niz" => "as_str",
        "obseg" => "span",
        "Zaporedje" => "Vec",
        "tok" => "stream",
        "potisni" => "push",
        "razširi" => "extend",
        "razmejevalnik" => "delimiter",
        "Ločilo" => "Punct",
        "Dobesedno" => "Literal",
        "postopkovni_makro" => "proc_macro",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rja(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
