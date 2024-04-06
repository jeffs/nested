/// Upper cases each line of UTF-8 text from r, and writesit to w.
fn to_upper<R: BufRead, W: Write>(
    mut r: R,
    w: &mut W,
) -> io::Result<()> {
    let line = &mut String::new();
    while r.read_line(line)? != 0 {
        let result = w.write(line.to_uppercase().as_bytes());
        if result.is_err() {
            let _ = w.flush();
            result?;
        }
    }
    Ok(())
}
