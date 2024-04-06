/// Upper cases text from r, and writes it to w.
fn to_upper<R: BufRead, W: Write>(
    mut r: R,
    w: &mut W,
) -> io::Result<()> {
    let line = &mut String::new();
    while r.read_line(line)? != 0 {
        w.write(line.to_uppercase().as_bytes())?;
    }
    Ok(())
}
