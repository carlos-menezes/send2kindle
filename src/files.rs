use std::io::Read;

pub(crate) fn is_pdf_file(content: &Vec<u8>) -> bool {
    content.starts_with(b"%PDF-")
}

pub(crate) fn read_from_stdin(mut buffer: &mut Vec<u8>) -> Result<(), std::io::Error> {
    std::io::stdin().read_to_end(&mut buffer)?;
    Ok(())
}

pub(crate) fn read_from_file(
    filename: &str,
    mut buffer: &mut Vec<u8>,
) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::open(filename)?;
    file.read_to_end(&mut buffer)?;
    Ok(())
}
