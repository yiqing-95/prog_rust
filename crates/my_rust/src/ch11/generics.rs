use std::io::Write;

fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> { out.write_all(b"hello world\n")?;
    out.flush()
}