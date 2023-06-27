//! Utilies for reading and scanning.

// Local imports
use super::*;

/// Size (in bytes) of the read/decode buffer array
const READER_BUFSIZE: usize = 65537;

/// A GDS reader.
///
/// Helper for parsing and scanning GDS coming from files and similar sources.
pub struct GdsReader {
    /// Read/conversion buffer.
    buf: [u8; READER_BUFSIZE],
    /// File being read.
    file: Cursor<Vec<u8>>, // FIXME: use &[u8], when we get around to piping around all the lifetimes.
}
impl GdsReader {
    /// Creates a [GdsReader], opening [File] at path `fname`.
    pub fn open(fname: impl AsRef<Path>) -> GdsResult<GdsReader> {
        let bytes = std::fs::read(fname)?;
        let cursor = Cursor::new(bytes);
        Ok(Self::new(cursor))
    }

    /// Creates a [GdsReader] of `bytes`.
    pub fn from_bytes(bytes: Vec<u8>) -> GdsReader {
        Self::new(Cursor::new(bytes))
    }

    /// Creates a [GdsReader] of `file`.
    pub fn new(file: Cursor<Vec<u8>>) -> GdsReader {
        let buf = [0; READER_BUFSIZE];
        GdsReader { file, buf }
    }

    /// Reads the next record-header from our file.
    ///
    /// Returns a [GdsRecordHeader] if successful.
    fn read_record_header(&mut self) -> GdsResult<GdsRecordHeader> {
        // Read the 16-bit record-size. (In bytes, including the four header bytes.)
        let len = match self.file.read_u16::<BigEndian>() {
            Err(e) => return Err(GdsError::Boxed(Arc::new(e))), // Reading error; raise it.
            Ok(num) if num < 4 => return Err(GdsError::RecordLen(num.into())), // Invalid (too short) length; throw Error.
            Ok(num) if num % 2 != 0 => return Err(GdsError::RecordLen(num.into())), // Invalid (odd) length; throw Error.
            Ok(num) => num, // The normal case
        };
        let len = len - 4; // Strip out the four header-bytes
                           // Read and decode its RecordType
        let record_type = self.file.read_u8()?;
        let record_type: GdsRecordType =
            FromPrimitive::from_u8(record_type).ok_or(GdsError::InvalidRecordType(record_type))?;
        if !record_type.valid() {
            return Err(GdsError::InvalidRecordType(record_type as u8));
        }
        // Read and decode its DataType
        let data_type = self.file.read_u8()?;
        let data_type =
            FromPrimitive::from_u8(data_type).ok_or(GdsError::InvalidDataType(data_type))?;
        Ok(GdsRecordHeader {
            rtype: record_type,
            dtype: data_type,
            len,
        })
    }

    /// Reads the next binary-encoded [GdsRecord].
    ///
    /// Returns a [GdsError] if `file` cursor is not on a record-boundary,
    /// or if binary decoding otherwise fails.
    fn read_record(&mut self) -> GdsResult<GdsRecord> {
        // Read the record header (types and length)
        let header = self.read_record_header()?;
        // And read the content
        self.read_record_content(&header)
    }

    fn read_record_content(&mut self, header: &GdsRecordHeader) -> GdsResult<GdsRecord> {
        // Based on that header-data, decode to a [GdsRecord]
        use GdsDataType::{BitArray, NoData, Str, F64, I16, I32};
        let len = header.len;
        let record: GdsRecord = match (header.rtype, header.dtype, len) {
            // Library-Level Records
            (GdsRecordType::Header, I16, 2) => GdsRecord::Header {
                version: self.read_i16(len)?[0],
            },
            (GdsRecordType::BgnLib, I16, 24) => GdsRecord::BgnLib {
                dates: self.read_i16(len)?,
            },
            (GdsRecordType::LibName, Str, _) => {
                GdsRecord::LibName(ArcStr::from(self.read_str(len)?))
            }
            (GdsRecordType::Units, F64, 16) => {
                let v = self.read_f64(len)?;
                GdsRecord::Units(v[0], v[1])
            }
            (GdsRecordType::EndLib, NoData, 0) => GdsRecord::EndLib,

            // Structure (Cell) Level Records
            (GdsRecordType::BgnStruct, I16, 24) => GdsRecord::BgnStruct {
                dates: self.read_i16(len)?,
            },
            (GdsRecordType::StructName, Str, _) => {
                GdsRecord::StructName(ArcStr::from(self.read_str(len)?))
            }
            (GdsRecordType::StructRefName, Str, _) => {
                GdsRecord::StructRefName(ArcStr::from(self.read_str(len)?))
            }
            (GdsRecordType::EndStruct, NoData, 0) => GdsRecord::EndStruct,

            // Element-Level Records
            (GdsRecordType::Boundary, NoData, 0) => GdsRecord::Boundary,
            (GdsRecordType::Path, NoData, 0) => GdsRecord::Path,
            (GdsRecordType::StructRef, NoData, 0) => GdsRecord::StructRef,
            (GdsRecordType::ArrayRef, NoData, 0) => GdsRecord::ArrayRef,
            (GdsRecordType::Text, NoData, 0) => GdsRecord::Text,
            (GdsRecordType::Layer, I16, 2) => GdsRecord::Layer(self.read_i16(len)?[0]),
            (GdsRecordType::DataType, I16, 2) => GdsRecord::DataType(self.read_i16(len)?[0]),
            (GdsRecordType::Width, I32, 4) => GdsRecord::Width(self.read_i32(len)?[0]),
            (GdsRecordType::Xy, I32, _) => GdsRecord::Xy(self.read_i32(len)?),
            (GdsRecordType::EndElement, NoData, 0) => GdsRecord::EndElement,

            // More (less well-categorized here) record-types
            (GdsRecordType::ColRow, I16, 4) => {
                let d = self.read_i16(len)?;
                GdsRecord::ColRow {
                    cols: d[0],
                    rows: d[1],
                }
            }
            (GdsRecordType::Node, NoData, 0) => GdsRecord::Node,
            (GdsRecordType::TextType, I16, 2) => GdsRecord::TextType(self.read_i16(len)?[0]),
            (GdsRecordType::Presentation, BitArray, 2) => {
                let bytes = self.read_bytes(len)?;
                GdsRecord::Presentation(bytes[0], bytes[1])
            }
            (GdsRecordType::String, Str, _) => GdsRecord::String(ArcStr::from(self.read_str(len)?)),
            (GdsRecordType::Strans, BitArray, 2) => {
                let bytes = self.read_bytes(len)?;
                GdsRecord::Strans(bytes[0], bytes[1])
            }
            (GdsRecordType::Mag, F64, 8) => GdsRecord::Mag(self.read_f64(len)?[0]),
            (GdsRecordType::Angle, F64, 8) => GdsRecord::Angle(self.read_f64(len)?[0]),
            (GdsRecordType::RefLibs, Str, _) => {
                GdsRecord::RefLibs(ArcStr::from(self.read_str(len)?))
            }
            (GdsRecordType::Fonts, Str, _) => GdsRecord::Fonts(ArcStr::from(self.read_str(len)?)),
            (GdsRecordType::PathType, I16, 2) => GdsRecord::PathType(self.read_i16(len)?[0]),
            (GdsRecordType::Generations, I16, 2) => GdsRecord::Generations(self.read_i16(len)?[0]),
            (GdsRecordType::AttrTable, Str, _) => {
                GdsRecord::AttrTable(ArcStr::from(self.read_str(len)?))
            }
            (GdsRecordType::ElemFlags, BitArray, 2) => {
                let bytes = self.read_bytes(len)?;
                GdsRecord::ElemFlags(bytes[0], bytes[1])
            }
            (GdsRecordType::Nodetype, I16, 2) => GdsRecord::Nodetype(self.read_i16(len)?[0]),
            (GdsRecordType::PropAttr, I16, 2) => GdsRecord::PropAttr(self.read_i16(len)?[0]),
            (GdsRecordType::PropValue, Str, _) => {
                GdsRecord::PropValue(ArcStr::from(self.read_str(len)?))
            }
            (GdsRecordType::Box, NoData, 0) => GdsRecord::Box,
            (GdsRecordType::BoxType, I16, 2) => GdsRecord::BoxType(self.read_i16(len)?[0]),
            (GdsRecordType::Plex, I32, 4) => GdsRecord::Plex(self.read_i32(len)?[0]),
            (GdsRecordType::BeginExtn, I32, 4) => GdsRecord::BeginExtn(self.read_i32(len)?[0]),
            (GdsRecordType::EndExtn, I32, 4) => GdsRecord::EndExtn(self.read_i32(len)?[0]),
            (GdsRecordType::TapeNum, I16, 2) => GdsRecord::TapeNum(self.read_i16(len)?[0]),
            (GdsRecordType::TapeCode, I16, 12) => GdsRecord::TapeCode(self.read_i16(len)?),
            (GdsRecordType::Format, I16, 2) => GdsRecord::Format(self.read_i16(len)?[0]),
            (GdsRecordType::Mask, Str, _) => GdsRecord::Mask(ArcStr::from(self.read_str(len)?)),
            (GdsRecordType::EndMasks, NoData, 0) => GdsRecord::EndMasks,
            (GdsRecordType::LibDirSize, I16, 2) => GdsRecord::LibDirSize(self.read_i16(len)?[0]),
            (GdsRecordType::SrfName, Str, _) => {
                GdsRecord::SrfName(ArcStr::from(self.read_str(len)?))
            }
            (GdsRecordType::LibSecur, I16, 2) => GdsRecord::LibSecur(self.read_i16(len)?[0]),

            // Failing to meet any of these clauses means this is an invalid record
            _ => return Err(GdsError::RecordDecode(header.rtype, header.dtype, len)),
        };
        Ok(record)
    }

    /// Reads `len` bytes and convert to `String`.
    fn read_str(&mut self, len: u16) -> GdsResult<String> {
        let len: usize = len.into();
        // ASCII Decode. First load bytes into our buffer.
        let mut data = &mut self.buf[0..len];
        self.file.read_exact(data)?;
        // Strip optional end-of-string chars
        let len = data.len();
        if data[len - 1] == 0x00 {
            data = &mut data[0..len - 1];
        }
        // And convert to string
        let s: String = std::str::from_utf8(data)?.into();
        Ok(s)
    }

    /// Reads `len` bytes.
    fn read_bytes(&mut self, len: u16) -> Result<Vec<u8>, std::io::Error> {
        let len: usize = len.into();
        let mut rv: Vec<u8> = vec![0; len];
        self.file.read_exact(&mut rv[0..len])?;
        Ok(rv)
    }

    /// Reads `len/2` i16s from `len` bytes.
    fn read_i16(&mut self, len: u16) -> Result<Vec<i16>, std::io::Error> {
        let len: usize = len.into();
        self.file.read_exact(&mut self.buf[0..len])?;
        let mut rv: Vec<i16> = vec![0; len / 2];
        (&self.buf[0..len]).read_i16_into::<BigEndian>(&mut rv)?;
        Ok(rv)
    }

    /// Reads `len/4` i32s from `len` bytes.
    fn read_i32(&mut self, len: u16) -> Result<Vec<i32>, std::io::Error> {
        let len: usize = len.into();
        self.file.read_exact(&mut self.buf[0..len])?;
        let mut rv: Vec<i32> = vec![0; len / 4];
        (&self.buf[0..len]).read_i32_into::<BigEndian>(&mut rv)?;
        Ok(rv)
    }

    /// Reads `len/8` f64s from `len` bytes, decoding GDS's float-format along the way.
    fn read_f64(&mut self, len: u16) -> GdsResult<Vec<f64>> {
        let len: usize = len.into();
        let mut u64s = vec![0; len / 8];
        self.file.read_u64_into::<BigEndian>(&mut u64s)?;
        let rv = u64s.into_iter().map(GdsFloat64::decode).collect();
        Ok(rv)
    }

    /// Gets the current file position.
    #[inline(always)]
    fn pos(&mut self) -> u64 {
        // Note `unwrap` here never panics, as [Cursor.stream_position] *always* returns `Ok`.
        self.file.stream_position().unwrap()
    }
}

/// A struct name and location detecting during a scan.
///
/// Typically generated by first-pass file scanning.
/// Stores a struct name and byte-offsets in its source file.
#[derive(Debug, Default)]
pub struct GdsStructScan {
    /// Struct name.
    name: String,
    /// Starting byte offset, at beginning of [BgnStruct](GdsRecordType::BgnStruct).
    #[allow(dead_code)]
    start: u64,
    /// Ending byte offset, at end of [EndStruct](GdsRecordType::EndStruct).
    end: u64,
}

/// A GDS scanner for finding [GdsStruct] definitions in a file,
///
/// Creates a first-pass list of detected cell names and byte-locations.
pub struct GdsScanner {
    /// Reader-helper.
    rdr: GdsReader,
    /// Next record-header, stored for peeking.
    nxt: GdsRecordHeader,
}

impl GdsScanner {
    /// Creates a new [GdsReader] iterator.
    pub fn new(mut rdr: GdsReader) -> GdsResult<Self> {
        // Decode the first record to initialize our "peeker".
        let nxt = rdr.read_record_header()?;
        Ok(Self { rdr, nxt })
    }

    /// Opens and scans structs in file `fname`.
    pub fn scan(fname: impl AsRef<Path>) -> GdsResult<Vec<GdsStructScan>> {
        let rdr = GdsReader::open(fname)?;
        let mut me = Self::new(rdr)?;
        me.scan_lib()
    }

    /// Expects/requires the next record to be of type `rtype`.
    ///
    /// Skips over the remainder of its content, if any.
    fn expect(&mut self, rtype: GdsRecordType) -> GdsResult<()> {
        if self.peek().rtype == rtype {
            // Success. Skip over the record and advance to the next.
            self.skip()
        } else {
            self.fail()
        }
    }

    /// Gets the next record's header, asserting it must be of type `rtype`.
    ///
    /// Consuming the remainder of the record is left to the caller.
    fn get(&mut self, rtype: GdsRecordType) -> GdsResult<&GdsRecordHeader> {
        if self.peek().rtype == rtype {
            Ok(self.peek())
        } else {
            self.fail()
        }
    }

    /// Advances our iterator and return the next element.
    fn next(&mut self) -> GdsResult<GdsRecordHeader> {
        if self.peek().rtype == GdsRecordType::EndLib {
            // Once we reach [EndLib], keep returning it forever.
            return Ok(*self.peek());
        }
        // Decode a new header and swap it with our `nxt`.
        let mut rv = self.rdr.read_record_header()?;
        mem::swap(&mut rv, &mut self.nxt);
        Ok(rv)
    }
    /// Skips over the current record's content, if any, and load the next.
    fn skip(&mut self) -> GdsResult<()> {
        let len = self.nxt.len.into();
        self.rdr.file.seek(SeekFrom::Current(len))?;
        self.next()?;
        Ok(())
    }

    /// Peeks at our next record, without advancing.
    #[inline(always)]
    fn peek(&self) -> &GdsRecordHeader {
        &self.nxt
    }

    /// Scans the GDS for cell/struct definitions.
    pub fn scan_lib(&mut self) -> GdsResult<Vec<GdsStructScan>> {
        // Read off header info
        self.expect(GdsRecordType::Header)?;
        self.expect(GdsRecordType::BgnLib)?;
        // Read the Library name
        let len = self.get(GdsRecordType::LibName)?.len;
        let _lib_name = self.rdr.read_str(len)?;
        self.next()?;
        // More header info
        self.expect(GdsRecordType::Units)?;

        // Scan all of the structs
        let mut strukts = Vec::<GdsStructScan>::with_capacity(1024);
        loop {
            let GdsRecordHeader { rtype, .. } = self.peek();
            match rtype {
                GdsRecordType::EndLib => break,
                GdsRecordType::BgnStruct => strukts.push(self.scan_struct()?),
                _ => return self.fail(),
            }
        }
        Ok(strukts)
    }

    /// Scans a single struct definition.
    ///
    /// Starts *after* the [BgnStruct] *header* has been read (just above).
    /// Returns a [GdsStructScan] if successful.
    fn scan_struct(&mut self) -> GdsResult<GdsStructScan> {
        // Create our scan-structure, and store the start-position of the struct.
        // Note this requires backing up the four header bytes.
        let mut s = GdsStructScan {
            start: self.pos() - 4,
            ..Default::default()
        };
        // Skip over the remainder of the [BeginStruct] header
        self.skip()?;

        // Read the Struct name
        let len = self.get(GdsRecordType::StructName)?.len;
        s.name = self.rdr.read_str(len)?;
        self.next()?;

        // Scan the struct content. Skip over everything until
        // [EndStruct](GdsRecordType::EndStruct).
        loop {
            let GdsRecordHeader { rtype, .. } = self.peek();
            match rtype {
                GdsRecordType::EndStruct => {
                    // Note the `len` of [EndStruct] is also zero, so we need not adjust the `end` position.
                    s.end = self.pos();
                    self.skip()?;
                    break;
                }
                // While *many* record-types are invalid here,
                // there's at least one we should check for: end-of-library,
                // Lest we find one and get stuck forever.
                GdsRecordType::EndLib => return self.fail(),
                // Everything else: skip and continue
                _ => self.skip()?,
            }
        }
        Ok(s)
    }

    /// Gets the current file position.
    #[inline(always)]
    fn pos(&mut self) -> u64 {
        self.rdr.pos()
    }

    /// Error generation helper.
    fn err(&mut self) -> GdsError {
        let pos = self.pos();
        GdsError::Str(format!(
            "Scanned Invalid Record {:?} at Byte Position {}",
            self.nxt.rtype, pos
        ))
    }

    /// Error generation helper.
    fn fail<T>(&mut self) -> GdsResult<T> {
        Err(self.err())
    }
}

/// A GDS parser.
///
/// A peekable iterator which loads GdsRecords from file, one at a time,
/// and converters them into a tree of Gds data structures.
pub struct GdsParser {
    /// File being read.
    rdr: GdsReader,
    /// Next record, stored for peeking.
    nxt: GdsRecord,
    /// Number of records read.
    numread: usize,
    /// Context stack.
    ctx: Vec<GdsContext>,
}

impl GdsParser {
    /// Creates a new GdsReader iterator for the file at path `fname`.
    pub fn open(fname: impl AsRef<Path>) -> GdsResult<GdsParser> {
        let rdr = GdsReader::open(fname)?;
        Self::new(rdr)
    }

    /// Creates a new GdsReader iterator for the byte-vector `bytes`.
    pub fn from_bytes(bytes: Vec<u8>) -> GdsResult<GdsParser> {
        let rdr = GdsReader::from_bytes(bytes);
        Self::new(rdr)
    }

    /// Creates a new GdsReader iterator.
    pub fn new(mut rdr: GdsReader) -> GdsResult<GdsParser> {
        // Decode the first record to initialize our "peeker"
        let nxt = rdr.read_record()?;
        Ok(GdsParser {
            rdr,
            nxt,
            numread: 1,
            ctx: Vec::new(),
        })
    }

    /// Advances our iterator and return the next element.
    fn next(&mut self) -> GdsResult<GdsRecord> {
        if self.nxt == GdsRecord::EndLib {
            // Once we reach [EndLib], keep returning it forever
            return Ok(GdsRecord::EndLib);
        }
        // Decode a new Record and swap it with our `nxt`
        let mut rv = self.rdr.read_record()?;
        mem::swap(&mut rv, &mut self.nxt);
        self.numread += 1;
        Ok(rv)
    }

    /// Peeks at our next record, without advancing.
    fn peek(&self) -> &GdsRecord {
        &self.nxt
    }

    /// Parses a [GdsLibrary]. Generally the start-state when reading a GDS file.
    pub fn parse_lib(&mut self) -> GdsResult<GdsLibrary> {
        self.ctx.push(GdsContext::Library);
        let mut lib = GdsLibraryBuilder::default();
        let mut structs = Vec::<GdsStruct>::with_capacity(1024);
        // Read the Header and its version data
        lib = match self.next()? {
            GdsRecord::Header { version: v } => lib.version(v),
            _ => return self.fail("Invalid library: missing GDS HEADER record"),
        };
        // Read the begin-lib
        lib = match self.next()? {
            GdsRecord::BgnLib { dates: d } => lib.dates(self.parse_datetimes(d)?),
            _ => return self.fail("Invalid library: missing GDS BGNLIB record"),
        };
        // Iterate over all others
        loop {
            let r = self.next()?;
            lib = match r {
                GdsRecord::EndLib => break, // End-of-library
                GdsRecord::LibName(d) => lib.name(d),
                GdsRecord::Units(d0, d1) => lib.units(GdsUnits(d0, d1)),
                GdsRecord::BgnStruct { dates } => {
                    let strukt = self.parse_struct(dates)?;
                    structs.push(strukt);
                    lib
                }
                // Spec-valid but unsupported records
                GdsRecord::LibDirSize(_)
                | GdsRecord::SrfName(_)
                | GdsRecord::LibSecur(_)
                | GdsRecord::RefLibs(_)
                | GdsRecord::Fonts(_)
                | GdsRecord::AttrTable(_)
                | GdsRecord::Generations(_)
                | GdsRecord::Format(_) => {
                    return Err(GdsError::Unsupported(Some(r), Some(GdsContext::Library)))
                }
                // Invalid
                _ => return self.invalid(r),
            };
        }
        // Add the Vec of structs, and create the Library from its builder
        lib = lib.structs(structs);
        Ok(lib.build()?)
    }

    /// Parses a cell ([GdsStruct]).
    fn parse_struct(&mut self, dates: Vec<i16>) -> GdsResult<GdsStruct> {
        self.ctx.push(GdsContext::Struct);
        let mut strukt = GdsStructBuilder::default();
        // Parse and store the header information: `dates` and `name`
        strukt = strukt.dates(self.parse_datetimes(dates)?);
        strukt = match self.next()? {
            GdsRecord::StructName(d) => strukt.name(d),
            _ => return self.fail("Missing Gds StructName"),
        };
        // Parse [GdsElement] records until hitting a [GdsRecord::EndStruct]
        let mut elems = Vec::<GdsElement>::with_capacity(1024);
        loop {
            let r = self.next()?;
            match r {
                GdsRecord::EndStruct => break, // End-of-struct
                GdsRecord::Boundary => elems.push(self.parse_boundary()?.into()),
                GdsRecord::Text => elems.push(self.parse_text_elem()?.into()),
                GdsRecord::Path => elems.push(self.parse_path()?.into()),
                GdsRecord::Box => elems.push(self.parse_box()?.into()),
                GdsRecord::StructRef => elems.push(self.parse_struct_ref()?.into()),
                GdsRecord::ArrayRef => elems.push(self.parse_array_ref()?.into()),
                GdsRecord::Node => elems.push(self.parse_node()?.into()),
                // Invalid
                _ => return self.invalid(r),
            };
        }
        strukt = strukt.elems(elems);
        let strukt = strukt.build()?;
        self.ctx.pop();
        Ok(strukt)
    }
    /// Parse a [GdsBoundary]
    fn parse_boundary(&mut self) -> GdsResult<GdsBoundary> {
        let mut b = GdsBoundaryBuilder::default();
        let mut props: Vec<GdsProperty> = Vec::new();

        loop {
            let r = self.next()?;
            b = match r {
                GdsRecord::EndElement => break, // End-of-element
                GdsRecord::Layer(d) => b.layer(d),
                GdsRecord::DataType(d) => b.datatype(d),
                GdsRecord::Xy(d) => b.xy(GdsPoint::parse_vec(&d)?),
                GdsRecord::Plex(d) => b.plex(GdsPlex(d)),
                GdsRecord::ElemFlags(d0, d1) => b.elflags(GdsElemFlags(d0, d1)),
                GdsRecord::PropAttr(attr) => {
                    props.push(self.parse_property(attr)?);
                    b
                }
                // Invalid
                _ => return self.invalid(r),
            };
        }
        b = b.properties(props);
        let b = b.build()?;
        self.ctx.pop();
        Ok(b)
    }

    /// Parses a [GdsPath].
    fn parse_path(&mut self) -> GdsResult<GdsPath> {
        self.ctx.push(GdsContext::Path);
        let mut b = GdsPathBuilder::default();
        let mut props: Vec<GdsProperty> = Vec::new();

        loop {
            let r = self.next()?;
            b = match r {
                GdsRecord::EndElement => break, // End-of-element
                GdsRecord::Layer(d) => b.layer(d),
                GdsRecord::DataType(d) => b.datatype(d),
                GdsRecord::Xy(d) => b.xy(GdsPoint::parse_vec(&d)?),
                GdsRecord::Width(d) => b.width(d),
                GdsRecord::PathType(d) => b.path_type(d),
                GdsRecord::BeginExtn(d) => b.begin_extn(d),
                GdsRecord::EndExtn(d) => b.end_extn(d),
                GdsRecord::Plex(d) => b.plex(GdsPlex(d)),
                GdsRecord::ElemFlags(d0, d1) => b.elflags(GdsElemFlags(d0, d1)),
                GdsRecord::PropAttr(attr) => {
                    props.push(self.parse_property(attr)?);
                    b
                }
                // Invalid
                _ => return self.invalid(r),
            };
        }
        b = b.properties(props);
        let b = b.build()?;
        self.ctx.pop();
        Ok(b)
    }

    /// Parses a [GdsTextElem] from an iterator of [GdsRecord]s.
    ///
    /// Requires the initial `Text` record has already been parsed.
    fn parse_text_elem(&mut self) -> GdsResult<GdsTextElem> {
        self.ctx.push(GdsContext::Text);
        let mut b = GdsTextElemBuilder::default();
        let mut props: Vec<GdsProperty> = Vec::new();

        loop {
            let r = self.next()?;
            b = match r {
                GdsRecord::EndElement => break, // End-of-element
                GdsRecord::Layer(d) => b.layer(d),
                GdsRecord::TextType(d) => b.texttype(d),
                GdsRecord::Xy(d) => b.xy(GdsPoint::parse(&d)?),
                GdsRecord::String(d) => b.string(d),
                GdsRecord::Presentation(d0, d1) => b.presentation(GdsPresentation(d0, d1)),
                GdsRecord::PathType(d) => b.path_type(d),
                GdsRecord::Width(d) => b.width(d),
                GdsRecord::Plex(d) => b.plex(GdsPlex(d)),
                GdsRecord::ElemFlags(d0, d1) => b.elflags(GdsElemFlags(d0, d1)),
                GdsRecord::Strans(d0, d1) => b.strans(self.parse_strans(d0, d1)?),
                GdsRecord::PropAttr(attr) => {
                    props.push(self.parse_property(attr)?);
                    b
                }
                // Invalid
                _ => return self.invalid(r),
            };
        }
        b = b.properties(props);
        let b = b.build()?;
        self.ctx.pop();
        Ok(b)
    }

    /// Parses a [GdsNode].
    fn parse_node(&mut self) -> GdsResult<GdsNode> {
        self.ctx.push(GdsContext::Node);
        let mut b = GdsNodeBuilder::default();
        let mut props: Vec<GdsProperty> = Vec::new();

        loop {
            let r = self.next()?;
            b = match r {
                GdsRecord::EndElement => break, // End-of-element
                GdsRecord::Layer(d) => b.layer(d),
                GdsRecord::Nodetype(d) => b.nodetype(d),
                GdsRecord::Xy(d) => b.xy(GdsPoint::parse_vec(&d)?),
                GdsRecord::Plex(d) => b.plex(GdsPlex(d)),
                GdsRecord::ElemFlags(d0, d1) => b.elflags(GdsElemFlags(d0, d1)),
                GdsRecord::PropAttr(attr) => {
                    props.push(self.parse_property(attr)?);
                    b
                }
                // Invalid
                _ => return self.invalid(r),
            };
        }
        b = b.properties(props);
        let b = b.build()?;
        self.ctx.pop();
        Ok(b)
    }

    /// Parses a [GdsBox].
    fn parse_box(&mut self) -> GdsResult<GdsBox> {
        self.ctx.push(GdsContext::Box);
        let mut b = GdsBoxBuilder::default();
        let mut props: Vec<GdsProperty> = Vec::new();

        loop {
            let r = self.next()?;
            b = match r {
                GdsRecord::EndElement => break, // End-of-element
                GdsRecord::Layer(d) => b.layer(d),
                GdsRecord::BoxType(d) => b.boxtype(d),
                GdsRecord::Xy(d) => {
                    // XY coordinates must be a five-element array.
                    // First parse a generic [GdsRecord::Xy] to a vector,
                    // and then convert, checking length in the process.
                    let v = GdsPoint::parse_vec(&d)?;
                    let xy: [GdsPoint; 5] = match v.try_into() {
                        Ok(xy) => xy,
                        Err(_) => return self.fail("Invalid XY for GdsBox"),
                    };
                    b.xy(xy)
                }
                GdsRecord::Plex(d) => b.plex(GdsPlex(d)),
                GdsRecord::ElemFlags(d0, d1) => b.elflags(GdsElemFlags(d0, d1)),
                GdsRecord::PropAttr(attr) => {
                    props.push(self.parse_property(attr)?);
                    b
                }
                // Invalid
                _ => return self.invalid(r),
            };
        }
        b = b.properties(props);
        let b = b.build()?;
        self.ctx.pop();
        Ok(b)
    }

    /// Parses a [GdsStructRef].
    fn parse_struct_ref(&mut self) -> GdsResult<GdsStructRef> {
        self.ctx.push(GdsContext::StructRef);
        let mut b = GdsStructRefBuilder::default();
        let mut props: Vec<GdsProperty> = Vec::new();

        loop {
            let r = self.next()?;
            b = match r {
                GdsRecord::EndElement => break, // End-of-element
                GdsRecord::StructRefName(d) => b.name(d),
                GdsRecord::Xy(d) => b.xy(GdsPoint::parse(&d)?),
                GdsRecord::Plex(d) => b.plex(GdsPlex(d)),
                GdsRecord::ElemFlags(d0, d1) => b.elflags(GdsElemFlags(d0, d1)),
                GdsRecord::Strans(d0, d1) => b.strans(self.parse_strans(d0, d1)?),
                GdsRecord::PropAttr(attr) => {
                    props.push(self.parse_property(attr)?);
                    b
                }
                // Invalid
                _ => return self.invalid(r),
            };
        }
        b = b.properties(props);
        let b = b.build()?;
        self.ctx.pop();
        Ok(b)
    }

    /// Parses a [GdsArrayRef].
    fn parse_array_ref(&mut self) -> GdsResult<GdsArrayRef> {
        self.ctx.push(GdsContext::ArrayRef);
        let mut b = GdsArrayRefBuilder::default();
        let mut props: Vec<GdsProperty> = Vec::new();

        loop {
            let r = self.next()?;
            b = match r {
                GdsRecord::EndElement => break, // End-of-element
                GdsRecord::StructRefName(d) => b.name(d),
                GdsRecord::ColRow { rows, cols } => b.rows(rows).cols(cols),
                GdsRecord::Xy(d) => {
                    // XY coordinates must be a three-element array.
                    // First parse a generic [GdsRecord::Xy] to a vector,
                    // and then convert, checking length in the process.
                    let v = GdsPoint::parse_vec(&d)?;
                    let xy: [GdsPoint; 3] = match v.try_into() {
                        Ok(xy) => xy,
                        Err(_) => return self.fail("Invalid XY for GdsArrayRef"),
                    };
                    b.xy(xy)
                }
                GdsRecord::Plex(d) => b.plex(GdsPlex(d)),
                GdsRecord::ElemFlags(d0, d1) => b.elflags(GdsElemFlags(d0, d1)),
                GdsRecord::Strans(d0, d1) => b.strans(self.parse_strans(d0, d1)?),
                GdsRecord::PropAttr(attr) => {
                    props.push(self.parse_property(attr)?);
                    b
                }
                // Invalid
                _ => return self.invalid(r),
            };
        }
        b = b.properties(props);
        let b = b.build()?;
        self.ctx.pop();
        Ok(b)
    }

    /// Parses a [GdsStrans] from records. Header bytes are passed as arguments `d0`, `d1`.
    fn parse_strans(&mut self, d0: u8, d1: u8) -> GdsResult<GdsStrans> {
        // Decode the first two bytes
        let mut s = GdsStrans {
            reflected: d0 & 0x80 != 0,
            abs_mag: d1 & 0x04 != 0,
            abs_angle: d1 & 0x02 != 0,
            ..Default::default()
        };
        // And parse optional magnitude & angle
        loop {
            match self.peek() {
                GdsRecord::Mag(d) => {
                    s.mag = Some(*d);
                    self.next()?; // Advance the iterator
                }
                GdsRecord::Angle(d) => {
                    s.angle = Some(*d);
                    self.next()?; // Advance the iterator
                }
                _ => break,
            }
        }
        Ok(s)
    }

    /// Parses a [GdsProperty].
    ///
    /// Numeric attribute `attr` is collected beforehand, as its record is the indication to parse an (attr, value) pair.
    fn parse_property(&mut self, attr: i16) -> GdsResult<GdsProperty> {
        self.ctx.push(GdsContext::Property);
        // `PropAttr` records must *immediately* be followed by `PropValue`, or parsing/ decoding fails.
        let value = if let GdsRecord::PropValue(v) = self.next()? {
            v
        } else {
            return self.fail("Gds Property without PropValue");
        };
        self.ctx.pop();
        Ok(GdsProperty { attr, value })
    }

    /// Parses from GDSII's vector of i16's format.
    fn parse_datetimes(&mut self, d: Vec<i16>) -> GdsResult<GdsDateTimes> {
        if d.len() != 12 {
            return self.fail("Invalid length GdsDateTimes");
        }
        Ok(GdsDateTimes {
            modified: NaiveDate::from_ymd_opt(d[0] as i32, d[1] as u32, d[2] as u32)
                .and_then(|ymd| ymd.and_hms_opt(d[3] as u32, d[4] as u32, d[5] as u32))
                .ok_or_else(|| self.err("Invalid modified time"))?,
            accessed: NaiveDate::from_ymd_opt(d[6] as i32, d[7] as u32, d[8] as u32)
                .and_then(|ymd| ymd.and_hms_opt(d[9] as u32, d[10] as u32, d[11] as u32))
                .ok_or_else(|| self.err("Invalid accessed time"))?,
        })
    }

    /// An error helper for an invalid record.
    fn invalid<T>(&mut self, record: GdsRecord) -> GdsResult<T> {
        Err(GdsError::Parse {
            msg: "Invalid GDS Record".into(),
            record,
            recordnum: self.numread,
            bytepos: self.rdr.pos(),
            ctx: self.ctx.clone(),
        })
    }

    /// Error helper that creates a Parse error.
    fn err(&mut self, msg: impl Into<String>) -> GdsError {
        GdsError::Parse {
            msg: msg.into(),
            record: self.peek().clone(), // FIXME: this will generally be one too far, sadly
            recordnum: self.numread,
            bytepos: self.rdr.pos(),
            ctx: self.ctx.clone(),
        }
    }

    /// Returns failure.
    fn fail<T>(&mut self, msg: impl Into<String>) -> GdsResult<T> {
        Err(self.err(msg))
    }

    /// JSON-serializes and writes all contents of the iterator to `writer`
    #[cfg(test)]
    pub fn write_records(&mut self, writer: &mut impl Write) -> GdsResult<()> {
        loop {
            let r = self.next()?;
            if r == GdsRecord::EndLib {
                return Ok(());
            }
            let entry: (usize, GdsRecord) = (self.numread, r);
            let s = serde_json::to_string(&entry).unwrap();
            write!(writer, "\t")?;
            writer.write_all(s.as_bytes()).unwrap();
            writeln!(writer, ",")?;
        }
    }

    /// Opens a GDS file `gds` and writes all [GdsRecord]s to JSON file `json`.
    #[cfg(test)]
    pub fn dump(gds: &str, json: &str) -> GdsResult<()> {
        // This streams one record at a time, rather than loading all into memory.
        // Create a ReaderIter from `gds`
        let mut me = Self::open(gds)?;
        // Create the JSON file
        let mut w = BufWriter::new(File::create(json)?);
        // Write it as a JSON list/sequence; add the opening bracket
        writeln!(w, "[")?;
        // Write all the records
        me.write_records(&mut w)?;
        // And close the list
        writeln!(w, "]")?;
        Ok(())
    }
}
