pub(super) fn count_neighbours(current: &[Vec<bool>], y: usize, x: usize) -> u8 {
    let mut count = 0;

    if let Some(slice) = current.get(y - 1) {
        if let Some(&cell) = slice.get(x - 1) {
            count += cell as u8;
        }
    }

    if let Some(slice) = current.get(y - 1) {
        if let Some(&cell) = slice.get(x) {
            count += cell as u8;
        }
    }

    if let Some(slice) = current.get(y - 1) {
        if let Some(&cell) = slice.get(x + 1) {
            count += cell as u8;
        }
    }

    if let Some(slice) = current.get(y) {
        if let Some(&cell) = slice.get(x - 1) {
            count += cell as u8;
        }
    }

    if let Some(slice) = current.get(y) {
        if let Some(&cell) = slice.get(x + 1) {
            count += cell as u8;
        }
    }

    if let Some(slice) = current.get(y + 1) {
        if let Some(&cell) = slice.get(x - 1) {
            count += cell as u8;
        }
    }

    if let Some(slice) = current.get(y + 1) {
        if let Some(&cell) = slice.get(x) {
            count += cell as u8;
        }
    }

    if let Some(slice) = current.get(y + 1) {
        if let Some(&cell) = slice.get(x + 1) {
            count += cell as u8;
        }
    }

    count
}
