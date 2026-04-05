use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, stdin};

use crate::error::{XcatError, XcatResult};

pub trait ReadableSource: Read {
    fn source_type(&self) -> &str;
}

pub enum InputSource {
    Stdin,
    File(File),
}

impl ReadableSource for InputSource {
    fn source_type(&self) -> &str {
        match self {
            InputSource::Stdin => "stdin",
            InputSource::File(_) => "file",
        }
    }
}

impl Read for InputSource {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            InputSource::Stdin => stdin().read(buf),
            InputSource::File(file) => file.read(buf),
        }
    }
}

pub struct FileReader {
    source: InputSource,
    path: String,
}

impl FileReader {
    pub fn open(path: &str) -> XcatResult<Self> {
        if path == "-" {
            Ok(Self {
                source: InputSource::Stdin,
                path: String::from("stdin"),
            })
        } else {
            let file = File::open(path).map_err(|e| XcatError::Io(e, path.to_string()))?;
            Ok(Self {
                source: InputSource::File(file),
                path: path.to_string(),
            })
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn source_type(&self) -> &str {
        self.source.source_type()
    }

    pub fn reader(&self) -> XcatResult<Box<dyn BufRead>> {
        match &self.source {
            InputSource::Stdin => Ok(Box::new(BufReader::new(stdin()))),
            InputSource::File(file) => Ok(Box::new(BufReader::new(file.try_clone().map_err(
                |e| XcatError::Io(e, self.path.clone()),
            )?))),
        }
    }

    pub fn read_to_string(&self) -> XcatResult<String> {
        let mut buf = String::new();
        match &self.source {
            InputSource::Stdin => {
                stdin()
                    .lock()
                    .read_to_string(&mut buf)
                    .map_err(|e| XcatError::Io(e, String::from("stdin")))?;
            }
            InputSource::File(_) => {
                let mut file =
                    File::open(&self.path).map_err(|e| XcatError::Io(e, self.path.clone()))?;
                file.read_to_string(&mut buf)
                    .map_err(|e| XcatError::Io(e, self.path.clone()))?;
            }
        }
        Ok(buf)
    }

    pub fn mmap_read(&self) -> XcatResult<Vec<u8>> {
        match &self.source {
            InputSource::Stdin => {
                let mut buf = Vec::new();
                stdin()
                    .lock()
                    .read_to_end(&mut buf)
                    .map_err(|e| XcatError::Io(e, String::from("stdin")))?;
                Ok(buf)
            }
            InputSource::File(_) => {
                let file =
                    File::open(&self.path).map_err(|e| XcatError::Io(e, self.path.clone()))?;
                let mmap = unsafe {
                    memmap2::Mmap::map(&file)
                        .map_err(|e| XcatError::Io(e.into(), self.path.clone()))?
                };
                Ok(mmap.to_vec())
            }
        }
    }

    pub fn line_iterator<'a>(&'a self) -> XcatResult<LineIterator<'a>> {
        let reader = self.reader()?;
        Ok(LineIterator {
            reader,
            line_num: 0,
        })
    }
}

pub struct LineIterator<'a> {
    reader: Box<dyn BufRead + 'a>,
    line_num: usize,
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = XcatResult<(usize, String)>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => {
                self.line_num += 1;
                let trimmed = line.strip_suffix('\n').unwrap_or(&line).to_string();
                Some(Ok((self.line_num, trimmed)))
            }
            Err(e) => Some(Err(XcatError::Io(e, String::from("read error")))),
        }
    }
}

pub fn process_line(
    line: &str,
    line_num: &mut usize,
    blank_lines: &mut usize,
    opts: &crate::display::DisplayOptions,
    colorizer: &crate::colorizer::Colorizer,
    out: &mut impl io::Write,
) -> XcatResult<()> {
    let is_blank = line.is_empty();

    if opts.squeeze_blank && is_blank {
        *blank_lines += 1;
        if *blank_lines > 1 {
            return Ok(());
        }
    } else {
        *blank_lines = 0;
    }

    if opts.number_lines() && (!opts.number_nonblank || !is_blank) {
        *line_num += 1;
        write!(out, "{}", colorizer.colorize_line_number(*line_num))?;
    }

    let mut processed = String::new();

    for ch in line.chars() {
        match ch {
            '\t' if opts.show_tabs => {
                processed.push_str(&colorizer.colorize_tab_marker());
            }
            c if c.is_control() && !matches!(c, '\n' | '\r') && opts.show_nonprinting => {
                let escaped = escape_control(c);
                processed.push_str(&colorizer.colorize_nonprint(&escaped));
            }
            c => processed.push(c),
        }
    }

    if opts.show_ends {
        processed.push_str(&colorizer.colorize_end_marker());
    }

    writeln!(out, "{processed}")?;
    Ok(())
}

fn escape_control(c: char) -> String {
    let code = c as u8;
    if code == 0x7F {
        return String::from("^?");
    }
    format!("^{}", (code + 0x40) as char)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colorizer::Colorizer;
    use crate::display::DisplayOptions;
    use std::io::Cursor;

    fn default_opts() -> DisplayOptions {
        DisplayOptions {
            number: false,
            number_nonblank: false,
            show_ends: false,
            squeeze_blank: false,
            show_tabs: false,
            show_nonprinting: false,
            no_color: false,
            color_enabled: false,
            count_lines: false,
        }
    }

    #[test]
    fn test_escape_control_char() {
        assert_eq!(escape_control('\x01'), "^A");
        assert_eq!(escape_control('\x00'), "^@");
        assert_eq!(escape_control('\x7F'), "^?");
        assert_eq!(escape_control('\x1B'), "^[");
    }

    #[test]
    fn test_process_line_plain() {
        let opts = default_opts();
        let colorizer = Colorizer::new(false);
        let mut out = Cursor::new(Vec::new());
        let mut line_num = 0;
        let mut blank = 0;

        process_line("hello", &mut line_num, &mut blank, &opts, &colorizer, &mut out)
            .unwrap();

        let result = String::from_utf8(out.into_inner()).unwrap();
        assert_eq!(result, "hello\n");
    }

    #[test]
    fn test_process_line_with_number() {
        let mut opts = default_opts();
        opts.number = true;
        let colorizer = Colorizer::new(false);
        let mut out = Cursor::new(Vec::new());
        let mut line_num = 0;
        let mut blank = 0;

        process_line("hello", &mut line_num, &mut blank, &opts, &colorizer, &mut out)
            .unwrap();

        let result = String::from_utf8(out.into_inner()).unwrap();
        assert_eq!(result, "     1\thello\n");
    }

    #[test]
    fn test_process_line_with_end_marker() {
        let mut opts = default_opts();
        opts.show_ends = true;
        let colorizer = Colorizer::new(false);
        let mut out = Cursor::new(Vec::new());
        let mut line_num = 0;
        let mut blank = 0;

        process_line("hello", &mut line_num, &mut blank, &opts, &colorizer, &mut out)
            .unwrap();

        let result = String::from_utf8(out.into_inner()).unwrap();
        assert_eq!(result, "hello$\n");
    }

    #[test]
    fn test_process_line_with_tab() {
        let mut opts = default_opts();
        opts.show_tabs = true;
        let colorizer = Colorizer::new(false);
        let mut out = Cursor::new(Vec::new());
        let mut line_num = 0;
        let mut blank = 0;

        process_line("a\tb", &mut line_num, &mut blank, &opts, &colorizer, &mut out)
            .unwrap();

        let result = String::from_utf8(out.into_inner()).unwrap();
        assert_eq!(result, "a^Ib\n");
    }

    #[test]
    fn test_process_line_squeeze_blank() {
        let mut opts = default_opts();
        opts.squeeze_blank = true;
        let colorizer = Colorizer::new(false);
        let mut out = Cursor::new(Vec::new());
        let mut line_num = 0;
        let mut blank = 0;

        process_line("", &mut line_num, &mut blank, &opts, &colorizer, &mut out).unwrap();
        process_line("", &mut line_num, &mut blank, &opts, &colorizer, &mut out).unwrap();
        process_line("", &mut line_num, &mut blank, &opts, &colorizer, &mut out).unwrap();

        let result = String::from_utf8(out.into_inner()).unwrap();
        assert_eq!(result, "\n");
    }

    #[test]
    fn test_process_line_nonprinting() {
        let mut opts = default_opts();
        opts.show_nonprinting = true;
        let colorizer = Colorizer::new(false);
        let mut out = Cursor::new(Vec::new());
        let mut line_num = 0;
        let mut blank = 0;

        process_line("a\x01b", &mut line_num, &mut blank, &opts, &colorizer, &mut out)
            .unwrap();

        let result = String::from_utf8(out.into_inner()).unwrap();
        assert_eq!(result, "a^Ab\n");
    }

    #[test]
    fn test_process_line_number_nonblank_skips_blank() {
        let mut opts = default_opts();
        opts.number_nonblank = true;
        let colorizer = Colorizer::new(false);
        let mut out = Cursor::new(Vec::new());
        let mut line_num = 0;
        let mut blank = 0;

        process_line("hello", &mut line_num, &mut blank, &opts, &colorizer, &mut out).unwrap();
        process_line("", &mut line_num, &mut blank, &opts, &colorizer, &mut out).unwrap();
        process_line("world", &mut line_num, &mut blank, &opts, &colorizer, &mut out).unwrap();

        let result = String::from_utf8(out.into_inner()).unwrap();
        assert_eq!(result, "     1\thello\n\n     2\tworld\n");
    }

    #[test]
    fn test_file_reader_stdin() {
        let reader = FileReader::open("-").unwrap();
        assert_eq!(reader.path(), "stdin");
    }

    #[test]
    fn test_file_reader_source_type() {
        let reader = FileReader::open("-").unwrap();
        assert_eq!(reader.source_type(), "stdin");
    }

    #[test]
    fn test_line_iterator_empty() {
        let content = "";
        let temp = std::fs::File::create("/tmp/xcat_test_empty.txt").unwrap();
        std::fs::write("/tmp/xcat_test_empty.txt", content).unwrap();

        let reader = FileReader::open("/tmp/xcat_test_empty.txt").unwrap();
        let iter = reader.line_iterator().unwrap();
        let lines: Vec<_> = iter.collect();
        assert!(lines.is_empty());
    }

    #[test]
    fn test_line_iterator_single_line() {
        std::fs::write("/tmp/xcat_test_single.txt", "hello\n").unwrap();
        let reader = FileReader::open("/tmp/xcat_test_single.txt").unwrap();
        let iter = reader.line_iterator().unwrap();
        let lines: Vec<_> = iter.collect();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].as_ref().unwrap().1, "hello");
        assert_eq!(lines[0].as_ref().unwrap().0, 1);
    }

    #[test]
    fn test_line_iterator_multiple_lines() {
        std::fs::write("/tmp/xcat_test_multi.txt", "line1\nline2\nline3\n").unwrap();
        let reader = FileReader::open("/tmp/xcat_test_multi.txt").unwrap();
        let iter = reader.line_iterator().unwrap();
        let lines: Vec<_> = iter.collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0].as_ref().unwrap().1, "line1");
        assert_eq!(lines[1].as_ref().unwrap().1, "line2");
        assert_eq!(lines[2].as_ref().unwrap().1, "line3");
    }

    #[test]
    fn test_mmap_read_small_file() {
        std::fs::write("/tmp/xcat_test_mmap.txt", "hello world\n").unwrap();
        let reader = FileReader::open("/tmp/xcat_test_mmap.txt").unwrap();
        let data = reader.mmap_read().unwrap();
        assert_eq!(data, b"hello world\n");
    }

    #[test]
    fn test_readable_source_trait() {
        let stdin_source = InputSource::Stdin;
        assert_eq!(stdin_source.source_type(), "stdin");
    }
}
