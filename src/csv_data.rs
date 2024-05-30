use clio::ClioPath;
use csv::StringRecord;
use ratatui::widgets::{Cell, Row};
use std::fmt::Debug;

use crate::InputReader;

pub struct CsvData {
    records: Vec<StringRecord>,
    column_lengths: Vec<usize>,
}

impl CsvData {
    pub fn records(&self) -> impl Iterator<Item = &StringRecord> {
        self.records.iter()
    }

    pub fn rows(&self) -> impl Iterator<Item = Row<'_>> {
        self.records
            .iter()
            .map(|r| Row::new(r.iter().map(Cell::new)))
    }

    pub fn record_window(&self, first: usize, count: usize) -> impl Iterator<Item = &StringRecord> {
        let upper_bound = usize::min(self.records.len(), first + count);
        self.records[first..upper_bound].iter()
    }

    pub fn row_window(&self, first: usize, count: usize) -> impl Iterator<Item = Row<'_>> {
        let upper_bound = usize::min(self.records.len(), first + count);
        self.records[first..upper_bound]
            .iter()
            .map(|r| Row::new(r.iter().map(Cell::new)))
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
    pub fn column_length(&self, col_idx: usize) -> Option<&usize> {
        self.column_lengths.get(col_idx)
    }
    pub fn column_lengths(&self) -> impl Iterator<Item = &usize> {
        self.column_lengths.iter()
    }
}

impl TryFrom<&ClioPath> for CsvData {
    type Error = anyhow::Error;

    fn try_from(path: &ClioPath) -> Result<Self, Self::Error> {
        let mut reader = csv::Reader::from_reader(InputReader::try_from(path)?);
        let mut records = Vec::new();
        let mut column_lengths = Vec::new();
        for record in reader.records() {
            let record = record?;

            if column_lengths.is_empty() {
                column_lengths = vec![0; record.iter().count()];
            } else {
                for (idx, s) in record.iter().enumerate() {
                    column_lengths[idx] = usize::max(column_lengths[idx], s.len());
                }
            }
            records.push(record);
        }
        Ok(Self {
            records,
            column_lengths,
        })
    }
}

impl Debug for CsvData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CsvData")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
