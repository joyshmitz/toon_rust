use crate::decode::parser::ArrayHeaderInfo;
use crate::decode::scanner::{BlankLineInfo, Depth, ParsedLine};
use crate::error::{Result, ToonError};
use crate::shared::constants::{COLON, LIST_ITEM_PREFIX};

/// Assert the expected count in strict mode.
///
/// # Errors
///
/// Returns an error when strict mode is enabled and counts differ.
pub fn assert_expected_count(
    actual: usize,
    expected: usize,
    item_type: &str,
    strict: bool,
) -> Result<()> {
    if strict && actual != expected {
        return Err(ToonError::message(format!(
            "Expected {expected} {item_type}, but got {actual}"
        )));
    }
    Ok(())
}

/// Validate that there are no extra list items beyond the expected count.
///
/// # Errors
///
/// Returns an error in strict mode when extra list items are found.
pub fn validate_no_extra_list_items(
    next_line: Option<&ParsedLine>,
    item_depth: Depth,
    expected_count: usize,
    strict: bool,
) -> Result<()> {
    if strict {
        if let Some(line) = next_line {
            if line.depth == item_depth && line.content.starts_with(LIST_ITEM_PREFIX) {
                return Err(ToonError::message(format!(
                    "Expected {expected_count} list array items, but found more"
                )));
            }
        }
    }
    Ok(())
}

/// Validate that there are no extra tabular rows beyond the expected count.
///
/// # Errors
///
/// Returns an error in strict mode when extra tabular rows are found.
pub fn validate_no_extra_tabular_rows(
    next_line: Option<&ParsedLine>,
    row_depth: Depth,
    header: &ArrayHeaderInfo,
    strict: bool,
) -> Result<()> {
    if strict {
        if let Some(line) = next_line {
            if line.depth == row_depth
                && !line.content.starts_with(LIST_ITEM_PREFIX)
                && is_data_row(&line.content, header.delimiter)
            {
                return Err(ToonError::message(format!(
                    "Expected {} tabular rows, but found more",
                    header.length
                )));
            }
        }
    }
    Ok(())
}

/// Validate that no blank lines appear within the specified range.
///
/// # Errors
///
/// Returns an error in strict mode when blank lines appear within the range.
pub fn validate_no_blank_lines_in_range(
    start_line: usize,
    end_line: usize,
    blank_lines: &[BlankLineInfo],
    strict: bool,
    context: &str,
) -> Result<()> {
    if !strict {
        return Ok(());
    }

    if let Some(first_blank) = blank_lines
        .iter()
        .find(|blank| blank.line_number > start_line && blank.line_number < end_line)
    {
        return Err(ToonError::message(format!(
            "Line {}: Blank lines inside {context} are not allowed in strict mode",
            first_blank.line_number
        )));
    }

    Ok(())
}

fn is_data_row(content: &str, delimiter: char) -> bool {
    let colon_pos = content.find(COLON);
    let delimiter_pos = content.find(delimiter);

    if colon_pos.is_none() {
        return true;
    }

    if let Some(delimiter_pos) = delimiter_pos {
        if let Some(colon_pos) = colon_pos {
            return delimiter_pos < colon_pos;
        }
    }

    false
}
