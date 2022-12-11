use std::{
    env::args_os,
    fmt::{self, Debug, Display, Formatter},
    fs::read_to_string,
    ops::{Index, IndexMut},
};

use color_eyre::{eyre::eyre, install, Result};

fn main() -> Result<()> {
    install()?;

    let file_path = args_os()
        .nth(1)
        .ok_or_else(|| eyre!("should have argument"))?;
    let contents = read_to_string(file_path)?;

    let num_rows = contents.lines().count();
    let num_columns = contents
        .lines()
        .next()
        .map(|row| row.chars().count())
        .ok_or_else(|| eyre!("should have row"))?;

    let mut grid = Grid::new(num_rows, num_columns);

    for (i, line) in contents.lines().enumerate() {
        for (j, tree) in line
            .chars()
            .map(|c| c.to_digit(10).ok_or_else(|| eyre!("should be digit")))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .enumerate()
        {
            grid[i][j] = tree;
        }
    }

    Ok(())
}

struct Grid<T> {
    num_columns: usize,
    num_rows: usize,
    data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    fn new(num_rows: usize, num_columns: usize) -> Self {
        Grid {
            num_columns,
            num_rows,
            data: vec![T::default(); num_rows * num_columns],
        }
    }

    fn num_rows(&self) -> usize {
        return self.num_rows;
    }

    fn num_columns(&self) -> usize {
        return self.num_columns;
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[(self.num_columns * index)..(self.num_columns * (index + 1))]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[(self.num_columns * index)..(self.num_columns * (index + 1))]
    }
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "\t[{:?}", &self[0])?;
        for row in 1..(self.num_rows - 1) {
            writeln!(f, "\t\t\t\t {:?}", &self[row])?;
        }
        write!(f, "\t\t\t\t {:?}]", &self[self.num_rows - 1])?;
        Ok(())
    }
}

impl<T> Display for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "[{:?}", &self[0])?;
        for row in 1..(self.num_rows - 1) {
            writeln!(f, " {:?}", &self[row])?;
        }
        write!(f, " {:?}]", &self[self.num_rows - 1])?;
        Ok(())
    }
}
