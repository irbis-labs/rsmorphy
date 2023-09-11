//! Метаинформация о расположении данных в памяти.
//!
//! Структура данных:
//!
//! | Смещение | Размер | Назначение |
//! |----------|--------|------------|
//! | 0 | 2 | Версия формата данных |
//! | 2 | 1 | Флаги, уточняющие особенности формата данных |
//! | 3 | 1 | Зарезервировано |
//! | 4 | 4 | Количество сегментов (элементов в массивах start и size) |
//! | 8 | 8 | Размер массива data в байтах |
//! | 16 | 48 | Зарезервировано |
//! | 64 | 4 * segments_number | Массив start |
//! | 64 + 4 * segments_number | 4 * segments_number | Массив size |
//! | 64 + 8 * segments_number | data_size | Массив data |
//!
//! В каком порядке сохраняются байты чисел всего массива данных должно определяться целевой
//! платформой, на которой будут использоваться данные. Например, если целевая платформа -
//! x86_64, то все числа должны быть сохранены в LittleEndian.
//!
//! Структура флагов:
//!
//! | Номер байта | Номер бита | Назначение |
//! |-------------|------------|------------|
//! | 0 | 0 | Если установлен, то все числа сохранены в BigEndian, иначе в LittleEndian. |
//! | 0 | 1 | Если установлен, то все числа в массивах start и size имеют тип u16, иначе u32. |
//! | 0 | 2 | Если установлен, то массивы start и size расположены в конце блока метаданных. |
//! | 0 | 3 | Если установлен, то значения в массиве start нужно умножать на 64. |
//!
//! Остальные биты зарезервированы.
//!
//! В обычном режиме на каждый блок данных выделяется минимум по 64 байта.
//! Метаданные, массивы start, size и data должны быть выровнены на границу 64 байт.
//!
//! В компактных режимах, в зависимости от флагов и количества элементов, массивы start и size
//! могут быть размещены в одном 64-байтном блоке, содержать в качестве элементов u16 вместо u32,
//! и могут быть размещены в конце блока метаданных.
//!
//! В одном 64-байтном блоке start и size размещаются в случаем, если их размер занимает до половины
//! блока. При этом массив start располагается в начале блока, а size - начиная с его середины
//! (выравнивание на 32 байта).
//!
//! При размещении в блоке метаданных, массивы start и size размещаются таким образом, чтобы
//! занять его конец, однако с выравниванием на 4 байта. Количество свободного места в блоке
//! метаданных может различаться в зависимости от версии формата данных, поэтому размещение
//! массивов в блоке метаданных указывается в флагах.

use std::io;
use std::io::Write;

use byteorder::ByteOrder;
use byteorder::WriteBytesExt;

/// Структура, описывающая расположение сохранённых данных.
#[derive(Debug, Clone, Copy)]
#[repr(C, align(64))]
pub struct SegmentsMeta {
    /// Версия формата данных.
    version: [u8; 2],
    /// Флаги, уточняющие особенности формата данных.
    flags: [u8; 1],
    _reserved1: [u8; 1],
    /// Количество элементов в массивах start и size.
    segments_number: u32,
    /// Размер массива data.
    data_size: u64,
    /// Зарезервировано.
    _reserved2: [u8; 48],
}

impl SegmentsMeta {
    pub fn new<Order: ByteOrder>(segments_number: u32, data_size: u64) -> Self {
        let is_big_endian = {
            let mut test = [0_u8; 2];
            Order::write_u16(&mut test[..], 0x0102);
            test == [1, 2]
        };

        let flags0 = [
            is_big_endian,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
        ]
        .into_iter()
        .rev()
        .fold(0, |acc, b| (acc << 1) | b as u8);

        SegmentsMeta {
            version: [0, 0],
            flags: [flags0],
            _reserved1: [0],
            segments_number,
            data_size,
            _reserved2: [0; 48],
        }
    }

    pub fn version(&self) -> &[u8; 2] {
        &self.version
    }

    pub fn is_big_endian(&self) -> bool {
        self.flags[0] & 0b_0000_0001 != 0
    }

    pub fn segments_number(&self) -> u32 {
        self.segments_number
    }

    pub fn data_size(&self) -> u64 {
        self.data_size
    }

    pub fn write<Order: ByteOrder>(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(&self.version)?;
        writer.write_all(&self.flags)?;
        writer.write_all(&self._reserved1)?;
        writer.write_u32::<Order>(self.segments_number)?;
        writer.write_u64::<Order>(self.data_size)?;
        writer.write_all(&self._reserved2)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use byteorder::BigEndian;

    use super::*;

    #[test]
    fn test_write() {
        let mut buf = [0_u8; 64];
        let mut cur = io::Cursor::new(&mut buf[..]);

        let meta = SegmentsMeta::new::<BigEndian>(12, 24);

        meta.write::<BigEndian>(&mut cur).unwrap();
        assert_eq!(cur.position(), 64);

        #[rustfmt::skip]
        let sample = [
            // version
            0, 0,
            // flags
            0b_0000_0001,
            // reserved1
            0,
            // segments_number
            0, 0, 0, 12,
            // data_size
            0, 0, 0, 0, 0, 0, 0, 24,
            // reserved2
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        assert_eq!(buf, sample);
    }
}
