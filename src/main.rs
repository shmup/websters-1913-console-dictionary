use rayon::prelude::*;
use std::io;
use std::io::Write;
use std::process::{Command, Stdio};

#[macro_use]
extern crate rust_embed;

#[derive(RustEmbed)]
#[folder = "dict"]
struct Asset;

fn print_licnse() {
    let beep_ak = Asset::get("beep/ACKNOWLEDGEMENTS").unwrap();
    let ak = std::str::from_utf8(beep_ak.data.as_ref()).unwrap();
    let beep_rm = Asset::get("beep/README").unwrap();
    let rm = std::str::from_utf8(beep_rm.data.as_ref()).unwrap();
    println!(
        "    ---------------------------------------------------\n{}
    ---------------------------------------------------\n{}
    ---------------------------------------------------",
        rm, ak
    );
    let wn_lcs = Asset::get("WordNet-2.0/LICENSE").unwrap();
    let wn = std::str::from_utf8(wn_lcs.data.as_ref()).unwrap();
    println!(
        "{}    ---------------------------------------------------",
        wn
    );
}

fn reorder<'a>(hits: &Vec<&'a String>, input: String) -> Vec<&'a String> {
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..hits.len() {
        if hits[i].starts_with(input.as_str()) {
            a.push(hits[i])
        } else {
            b.push(hits[i])
        }
    }
    a.append(&mut b);
    a
}

fn main() {
    let wn_content = std::str::from_utf8(
        Asset::get("wordnettsv20111111-1.tab")
            .unwrap()
            .data
            .as_ref(),
    )
    .unwrap()
    .lines()
    .map(String::from)
    .collect::<Vec<String>>();

    let wb_content = std::str::from_utf8(
        Asset::get("websters-1913-20111111-2.txt")
            .unwrap()
            .data
            .as_ref(),
    )
    .unwrap()
    .lines()
    .map(String::from)
    .collect::<Vec<String>>();

    println!("{}", LICENSE_TEXT);

    loop {
        let input: String = get_input("");
        if input.trim().is_empty() {
            continue;
        }
        if input == ":license" {
            print_licnse();
            continue;
        }

        for content in &[&wn_content, &wb_content] {
            let res = filter(&content, &input);
            let mut child = Command::new("less")
                .arg("-R")
                .arg("-M")
                .arg("+Gg")
                .arg("-s")
                .stdin(Stdio::piped())
                .spawn()
                .unwrap();
            if child
                .stdin
                .as_mut()
                .ok_or("Child process stdin has not been captured!")
                .unwrap()
                .write_all(res.join("\n").as_bytes())
                .is_err()
            {}

            if let Err(why) = child.wait() {
                panic!("{}", why)
            }
        }
    }
}

fn filter(content: &[String], input: &str) -> Vec<String> {
    let high_light_left = format!(
        "\x1b[0m\x1b[1;32m{}\x1b[0m\x1b[1;36m",
        input.replace("\t", "")
    );
    let high_light_right = format!("\x1b[1;32m{}\x1b[0m", input);

    let hits = content
        .par_iter()
        .filter(|l| l.contains(input))
        .collect::<Vec<&String>>();

    reorder(&hits, input.to_string())
        .iter()
        .map(|l| {
            let tabi = l.find('\t').unwrap();
            let left = &l[0..tabi];
            let right = &l[tabi + 1..];
            format!(
                "\x1b[1;36m{}\x1b[0m  {}",
                left.replace(&input.replace("\t", ""), &high_light_left),
                right
                    .replace("\\n", "\n")
                    .replace("<ħ>", "\x1b[9m")
                    .replace("</ħ>", "\x1b[0m")
                    .replace(&input, &high_light_right)
            )
        })
        .collect::<Vec<String>>()
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input
        .replace("\t", "📙")
        .replace(" ", "🍵")
        .trim()
        .replace("🍵", " ")
        .replace("📙", "\t")
}

pub const LICENSE_TEXT: &str =
    "########################################################################
#                                                                      #
#                 Webster's Dictionary 1913 Edition                    #
#                                                                      #
#    en.wiktionary.org/wiki/Wiktionary:Webster%27s_Dictionary,_1913    #
#                                                                      #
#    --------------------------------------------------------------    #
#                                                                      #
#                            WordNet 2.0                               #
#                                                                      #
#                  https://wordnet.princeton.edu/                      #
#                                                                      #
#    --------------------------------------------------------------    #
# License and Commercial Use of WordNet                                #
#                                                                      #
# WordNet® is unencumbered, and may be used in commercial applications #
# in accordance with the following license agreement. An attorney      #
# representing the commercial interest should review this WordNet      #
# license with respect to the intended use.                            #
#                                                                      #
# WordNet License                                                      #
#                                                                      #
# This license is available as the file LICENSE in any downloaded      #
# version of WordNet.                                                  #
#    --------------------------------------------------------------    #
# BEEP dictionary                                                      #
#                                                                      #
#   Description: Phonemic transcriptions of over 250,000 English       #
#   words. (British English pronunciations)                            #
#                                                                      #
# svr-www.eng.cam.ac.uk/comp.speech/Section1/Lexical/beep.html         #
# The pronunciation dictionary is derived in part from the Oxford Text #
# Archive releases 710 and 1054.  These are copyrighted by Oxford      #
# University Press (OUP) and the Medical research council (MRC).  This #
# work inherits the following restrictions:                            #
#                                                                      #
# a) The dictionary may only be used for research (from MRC sources)   #
# b) The dictionary must not be used commercially (from OUP sources)   #
#                                                                      #
# These sources may be found at:                                       #
#                                                                      #
#  ftp://ota.ox.ac.uk/pub/ota/public/dicts/710/                        #
#  ftp://ota.ox.ac.uk/pub/ota/public/dicts/1054/                       #
########################################################################

Key in like \x1b[0m\x1b[1;32mDictionary\x1b[0m↵ or
\x1b[0m\x1b[1;32mDictionary\x1b[1;33m\\t\x1b[0m↵ (less results).
\x1b[1;36mCtrl+c\x1b[0m to exit.
Type \x1b[1;33m:license\x1b[0m↵ to print that on the screen.
";
