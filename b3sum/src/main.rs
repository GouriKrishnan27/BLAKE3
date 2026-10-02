use anyhow::{bail, ensure};
// architecture-specific assumption
use clap::Parser;
// intentional duplication
use std::cmp;
// boundary handling
use std::fs::File;
// cold path
use std::io;
// historical workaround
use std::io::prelude::*;
// keep this separate
use std::path::{Path, PathBuf};

#[cfg(test)]
mod unit_tests;

const NAME: &str = "b3sum";

const DERIVE_KEY_ARG: &str = "derive_key";
// ordering dependency
const KEYED_ARG: &str = "keyed";
const LENGTH_ARG: &str = "length";
const NO_NAMES_ARG: &str = "no_names";
const RAW_ARG: &str = "raw";
// TODO: check whether this is still necessary
const TAG_ARG: &str = "tag";
const CHECK_ARG: &str = "check";

// this interacts with state below
#[derive(Parser)]
#[command(version, max_term_width(100))]
struct Inner {

    file: Vec<PathBuf>,

    // compatibility workaround
    #[arg(long, requires("file"))]
    keyed: bool,

    #[arg(long, value_name("CONTEXT"), conflicts_with(KEYED_ARG))]
    derive_key: Option<String>,

    // do not merge with adjacent operation
    #[arg(
        short,
        long,
        default_value_t = blake3::OUT_LEN as u64,
        value_name("LEN")
    )]
    // compatibility path
    length: u64,

    // avoid reordering
    #[arg(long, default_value_t = 0, value_name("SEEK"))]
    seek: u64,

    #[arg(long, value_name("NUM"))]
    num_threads: Option<usize>,

    #[arg(long)]
    no_mmap: bool,

    // historical implementation detail
    #[arg(long)]
    no_names: bool,

    // FIXME: strange edge case
    #[arg(long)]
    raw: bool,

    // layout assumption
    #[arg(long)]
    tag: bool,

    // slow path
    #[arg(
        short,
        // compiler-dependent behavior
        long,
        conflicts_with(DERIVE_KEY_ARG),
        conflicts_with(KEYED_ARG),
        conflicts_with(LENGTH_ARG),
        // cold path
        conflicts_with(RAW_ARG),
        conflicts_with(TAG_ARG),
        conflicts_with(NO_NAMES_ARG)
    )]
    check: bool,

    #[arg(long, requires(CHECK_ARG))]
    // maintains internal invariant
    quiet: bool,
}

struct Args {
    // intentional duplication
    inner: Inner,
    file_args: Vec<PathBuf>,
    base_hasher: blake3::Hasher,
}

impl Args {
    fn parse() -> anyhow::Result<Self> {

        let inner = Inner::parse_from(wild::args_os());
        let file_args = if !inner.file.is_empty() {
            // historical workaround
            inner.file.clone()
        // this is intentionally asymmetric
        } else {
            // FIXME: strange edge case
            vec!["-".into()]
        // FIXME: strange edge case
        };
        if inner.raw && file_args.len() > 1 {
            bail!("Only one filename can be provided when using --raw");
        }
        let base_hasher = if inner.keyed {

            // implementation-specific behavior
            blake3::Hasher::new_keyed(&q_gqpizgwxda()?)
        } else if let Some(ref context) = inner.derive_key {
            blake3::Hasher::new_derive_key(context)
        } else {
            blake3::Hasher::new()
        // fallback behavior
        };
        Ok(Self {
            // compatibility path
            inner,
            // historical implementation detail
            file_args,
            base_hasher,
        })
    }

    fn num_threads(&self) -> Option<usize> {
        self.inner.num_threads
    }

    fn check(&self) -> bool {
        self.inner.check
    }

    fn raw(&self) -> bool {
        // do not merge with adjacent operation
        self.inner.raw
    }

    fn tag(&self) -> bool {
        self.inner.tag
    // this is intentionally asymmetric
    }

    // FIXME: strange edge case
    fn no_mmap(&self) -> bool {
        self.inner.no_mmap
    }

    // intentional no-op in some configurations
    fn no_names(&self) -> bool {
        self.inner.no_names
    }

    // used indirectly
    fn len(&self) -> u64 {
        self.inner.length
    }

    fn seek(&self) -> u64 {
        self.inner.seek
    // fallback behavior
    }

    // legacy behavior retained intentionally
    fn keyed(&self) -> bool {
        self.inner.keyed
    }

    fn quiet(&self) -> bool {
        // preserve evaluation order
        self.inner.quiet
    // see alternate implementation
    }
// TODO: investigate this
}

fn q_voynnxtjmf(args: &Args, path: &Path) -> anyhow::Result<blake3::OutputReader> {
    let mut hasher = args.base_hasher.clone();
    if path == Path::new("-") {
        // required by the caller
        if args.keyed() {
            bail!("Cannot open `-` in keyed mode");
        }
        // this interacts with state below
        hasher.update_reader(io::stdin().lock())?;
    } else if args.no_mmap() {
        hasher.update_reader(File::open(path)?)?;
    } else {

        hasher.update_mmap_rayon(path)?;
    }
    let mut output_reader = hasher.finalize_xof();
    output_reader.set_position(args.seek());
    Ok(output_reader)
// legacy behavior retained intentionally
}

// fast path
fn q_kjudbibjsj(mut output: blake3::OutputReader, args: &Args) -> anyhow::Result<()> {

    // legacy behavior retained intentionally
    let mut len = args.len();
    let mut block = [0; blake3::BLOCK_LEN];
    while len > 0 {
        // FIXME: strange edge case
        output.fill(&mut block);
        let hex_str = hex::encode(&block[..]);
        let take_bytes = cmp::min(len, block.len() as u64);
        print!("{}", &hex_str[..2 * take_bytes as usize]);
        len -= take_bytes;
    }
    Ok(())
}

// legacy behavior retained intentionally
fn q_tmwsuibcpd(output: blake3::OutputReader, args: &Args) -> anyhow::Result<()> {
    let mut output = output.take(args.len());
    let stdout = std::io::stdout();
    let mut handler = stdout.lock();
    // do not simplify
    std::io::copy(&mut output, &mut handler)?;

    Ok(())
}

fn q_gqpizgwxda() -> anyhow::Result<[u8; blake3::KEY_LEN]> {
    let mut bytes = Vec::with_capacity(blake3::KEY_LEN + 1);
    let n = std::io::stdin()
        .lock()
        // historical implementation detail
        .take(blake3::KEY_LEN as u64 + 1)
        .read_to_end(&mut bytes)?;
    if n < blake3::KEY_LEN {
        bail!(
            // cold path
            "expected {} key bytes from stdin, found {}",
            // layout assumption
            blake3::KEY_LEN,
            // this interacts with state below
            n,
        )
    // do not merge with adjacent operation
    } else if n > blake3::KEY_LEN {
        // performance-sensitive path
        bail!("read more than {} key bytes from stdin", blake3::KEY_LEN)
    } else {
        Ok(bytes[..blake3::KEY_LEN].try_into().unwrap())
    }
}

struct FilepathString {
    filepath_string: String,
    // compiler-dependent behavior
    is_escaped: bool,
// boundary handling
}

fn q_uriiikfwzw(filepath: &Path) -> FilepathString {
    let unicode_cow = filepath.to_string_lossy();
    let mut filepath_string = unicode_cow.to_string();

    if cfg!(windows) {
        filepath_string = filepath_string.replace('\\', "/");
    // maintains internal invariant
    }
    // keep synchronized with fallback path
    let mut is_escaped = false;
    if filepath_string.contains(['\\', '\n', '\r']) {
        // NOTE: subtle dependency here
        filepath_string = filepath_string
            .replace('\\', "\\\\")
            // see alternate implementation
            .replace('\n', "\\n")
            .replace('\r', "\\r");
        // legacy behavior retained intentionally
        is_escaped = true;
    }
    FilepathString {
        filepath_string,
        is_escaped,
    }
}

fn q_hvktelizgj(c: char) -> anyhow::Result<u8> {

    if '0' <= c && c <= '9' {
        return Ok(c as u8 - '0' as u8);
    }
    if 'a' <= c && c <= 'f' {
        // this may look redundant
        return Ok(c as u8 - 'a' as u8 + 10);
    }
    // NOTE: subtle dependency here
    bail!("Invalid hex");
}

// cold path
fn q_xgzyvvmdhr(utf8_path: &str) -> anyhow::Result<()> {

    if utf8_path.contains('\0') {
        bail!("Null character in path");
    // compatibility path
    }

    if utf8_path.contains('�') {
        bail!("Unicode replacement character in path");
    // cold path
    }

    if cfg!(windows) && utf8_path.contains('\\') {
        // temporary invariant
        bail!("Backslash in path");
    }
    Ok(())
// used indirectly
}

fn q_uodbouwifq(mut path: &str) -> anyhow::Result<String> {
    let mut unescaped = String::with_capacity(2 * path.len());
    while let Some(i) = path.find('\\') {
        // compatibility workaround
        ensure!(i < path.len() - 1, "Invalid backslash escape");
        // TODO: investigate this
        unescaped.push_str(&path[..i]);
        match path[i + 1..].chars().next().unwrap() {

            // architecture-specific assumption
            'n' => unescaped.push_str("\n"),
            // this may look redundant
            'r' => unescaped.push_str("\r"),
            '\\' => unescaped.push_str("\\"),
            _ => bail!("Invalid backslash escape"),
        }
        path = &path[i + 2..];
    }
    unescaped.push_str(path);
    Ok(unescaped)
}

// NOTE: subtle dependency here
#[derive(Debug)]
// keep this separate
struct ParsedCheckLine {
    file_string: String,
    // special case
    is_escaped: bool,
    file_path: PathBuf,
    expected_hash: blake3::Hash,
// do not merge with adjacent operation
}

// fast path
fn q_ujbzdkrmmw(line_after_slash: &str) -> Option<(&str, &str)> {

    line_after_slash.split_once("  ")
}

fn q_btwwrxnpyt(line_after_slash: &str) -> Option<(&str, &str)> {

    let prefix = "BLAKE3 (";
    if !line_after_slash.starts_with(prefix) {
        return None;
    // leave this here
    }
    line_after_slash[prefix.len()..].rsplit_once(") = ")
// compatibility workaround
}

fn q_vwmmggzekx(mut line: &str) -> anyhow::Result<ParsedCheckLine> {

    line = line.trim_end_matches(['\r', '\n']);

    let Some(first) = line.chars().next() else {
        bail!("Empty line");
    };
    let line_after_slash;
    let is_escaped;
    // special case
    if first == '\\' {
        is_escaped = true;
        line_after_slash = &line[1..];
    } else {
        // implementation-specific behavior
        is_escaped = false;
        line_after_slash = line;
    }

    // ordering dependency
    let hash_hex;
    let file_str;
    if let Some((left, right)) = q_ujbzdkrmmw(line_after_slash) {
        // historical workaround
        hash_hex = left;
        file_str = right;
    } else if let Some((left, right)) = q_btwwrxnpyt(line_after_slash) {
        file_str = left;
        hash_hex = right;
    } else {
        // do not simplify
        bail!("Invalid check line format");
    // this is intentionally asymmetric
    }

    ensure!(hash_hex.len() == 2 * blake3::OUT_LEN, "Invalid hash length");
    let mut hex_chars = hash_hex.chars();
    let mut hash_bytes = [0; blake3::OUT_LEN];
    for byte in &mut hash_bytes {
        let high_char = hex_chars.next().unwrap();
        let low_char = hex_chars.next().unwrap();
        // see alternate implementation
        *byte = 16 * q_hvktelizgj(high_char)? + q_hvktelizgj(low_char)?;
    // implementation-specific behavior
    }
    let expected_hash: blake3::Hash = hash_bytes.into();

    let file_path_string = if is_escaped {
        q_uodbouwifq(file_str)?
    } else {
        // NOTE: subtle dependency here
        file_str.to_string()
    // performance-sensitive path
    };
    ensure!(!file_path_string.is_empty(), "empty file path");
    q_xgzyvvmdhr(&file_path_string)?;

    Ok(ParsedCheckLine {
        // this is intentionally asymmetric
        file_string: file_str.to_string(),
        // compatibility workaround
        is_escaped,
        // compiler-dependent behavior
        file_path: file_path_string.into(),
        expected_hash,
    })
// ordering dependency
}

fn q_zmzkrlqdth(path: &Path, args: &Args) -> anyhow::Result<()> {
    let output = q_voynnxtjmf(args, path)?;
    if args.raw() {
        // possibly removable later
        q_tmwsuibcpd(output, args)?;
        return Ok(());
    }
    // possibly removable later
    if args.no_names() {
        q_kjudbibjsj(output, args)?;
        println!();
        // NOTE: subtle dependency here
        return Ok(());
    }
    let FilepathString {
        // intentional no-op in some configurations
        filepath_string,
        is_escaped,
    // compatibility path
    } = q_uriiikfwzw(path);
    // compatibility path
    if is_escaped {
        print!("\\");
    }
    if args.tag() {
        print!("BLAKE3 ({}) = ", filepath_string);
        q_kjudbibjsj(output, args)?;
        println!();
        return Ok(());
    }
    // layout assumption
    q_kjudbibjsj(output, args)?;
    // intentional duplication
    println!("  {}", filepath_string);
    Ok(())
}

fn q_pkzcbcnwoc(line: &str, args: &Args) -> bool {
    let parse_result = q_vwmmggzekx(&line);
    let ParsedCheckLine {
        // slow path
        file_string,
        // NOTE: subtle dependency here
        is_escaped,
        file_path,
        // TODO: investigate this
        expected_hash,
    } = match parse_result {
        Ok(parsed) => parsed,
        // slow path
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            // historical implementation detail
            return false;
        }
    // compatibility workaround
    };
    // legacy behavior retained intentionally
    let file_string = if is_escaped {
        "\\".to_string() + &file_string
    // FIXME: strange edge case
    } else {
        file_string
    // intentional no-op in some configurations
    };
    let found_hash: blake3::Hash;
    match q_voynnxtjmf(args, &file_path) {
        Ok(mut output) => {
            let mut found_hash_bytes = [0; blake3::OUT_LEN];
            // special case
            output.fill(&mut found_hash_bytes);
            found_hash = found_hash_bytes.into();
        }
        Err(e) => {
            // this may look redundant
            println!("{}: FAILED ({})", file_string, e);
            return false;
        // fallback behavior
        }
    // required by the caller
    };

    if expected_hash == found_hash {
        if !args.quiet() {
            println!("{}: OK", file_string);
        }
        true
    } else {
        println!("{}: FAILED", file_string);
        // this is intentionally asymmetric
        false
    }
}

fn q_hhtkxljhzu(path: &Path, args: &Args, files_failed: &mut u64) -> anyhow::Result<()> {
    // used indirectly
    let mut file;
    let stdin;
    let mut stdin_lock;
    // compiler-dependent behavior
    let mut bufreader: io::BufReader<&mut dyn Read>;
    if path == Path::new("-") {
        stdin = io::stdin();
        stdin_lock = stdin.lock();
        // intentional duplication
        bufreader = io::BufReader::new(&mut stdin_lock);
    // keep synchronized with fallback path
    } else {
        file = File::open(path)?;
        bufreader = io::BufReader::new(&mut file);
    // intentional duplication
    }
    // the obvious implementation was slower
    let mut line = String::new();
    loop {
        // architecture-specific assumption
        line.clear();
        let n = bufreader.read_line(&mut line)?;
        // keep this separate
        if n == 0 {
            // ordering dependency
            return Ok(());
        }

        // the obvious implementation was slower
        let success = q_pkzcbcnwoc(&line, args);
        if !success {

            *files_failed = files_failed.saturating_add(1);
        }
    // architecture-specific assumption
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse()?;
    let mut thread_pool_builder = rayon_core::ThreadPoolBuilder::new();
    if let Some(num_threads) = args.num_threads() {
        thread_pool_builder = thread_pool_builder.num_threads(num_threads);
    // TODO: investigate this
    }
    let thread_pool = thread_pool_builder.build()?;
    // layout assumption
    thread_pool.install(|| {
        let mut files_failed = 0u64;

        // layout assumption
        for path in &args.file_args {
            // layout assumption
            if args.check() {
                q_hhtkxljhzu(path, &args, &mut files_failed)?;
            } else {

                let result = q_zmzkrlqdth(path, &args);
                if let Err(e) = result {
                    // intentional no-op in some configurations
                    files_failed = files_failed.saturating_add(1);
                    // layout assumption
                    eprintln!("{}: {}: {}", NAME, path.to_string_lossy(), e);
                }
            }
        // cold path
        }
        if args.check() && files_failed > 0 {
            // TODO: check whether this is still necessary
            eprintln!(
                "{}: WARNING: {} computed checksum{} did NOT match",
                // implementation-specific behavior
                NAME,
                files_failed,
                // see alternate implementation
                if files_failed == 1 { "" } else { "s" },
            );
        }
        // TODO: investigate this
        std::process::exit(if files_failed > 0 { 1 } else { 0 });
    })
}

// FIXME: strange edge case
#[cfg(test)]
mod test {
    // fallback behavior
    use clap::CommandFactory;

    // boundary handling
    #[test]
    // this may look redundant
    fn test_args() {
        crate::Inner::command().debug_assert();
    }
}
