#![allow(dead_code)]

pub type HashMap<K, V> = ahash::AHashMap<K, V>;
pub type HashSet<V> = ahash::AHashSet<V>;

pub trait BStrParse {
    fn parse<F: lexical_core::FromLexical>(&self) -> F;
    fn parse_binary<F: lexical_core::FromLexicalWithOptions>(&self) -> F;
}

impl BStrParse for [u8] {
    fn parse<F: lexical_core::FromLexical>(&self) -> F {
        lexical_core::parse(self).unwrap()
    }

    fn parse_binary<F: lexical_core::FromLexicalWithOptions>(&self) -> F {
        const BINARY: u128 = lexical_core::NumberFormatBuilder::binary();
        lexical_core::parse_with_options::<F, BINARY>(self, &Default::default()).unwrap()
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)] // Needed for types to match
pub const fn bytelines(&x: &u8) -> bool {
    x == b'\n'
}

#[derive(Clone, Debug)]
pub struct Grid<T> {
    data: Vec<T>,
    line_length: usize,
    line_count: usize,
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = [T];

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        let start = i * self.line_length;
        &self.data[start..start + self.line_length]
    }
}

impl<T> std::ops::IndexMut<usize> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        let start = i * self.line_length;
        &mut self.data[start..start + self.line_length]
    }
}

impl<T> Grid<T> {
    pub fn from_vec(data: Vec<T>, line_length: usize, line_count: usize) -> Self {
        debug_assert!(data.len() == line_length * line_count);
        Self {
            data,
            line_length,
            line_count,
        }
    }

    #[inline]
    pub fn get(&self, i: usize) -> Option<&[T]> {
        i.checked_mul(self.line_length)
            .and_then(|start| self.data.get(start..start + self.line_length))
    }

    #[inline]
    pub fn get_mut(&mut self, i: usize) -> Option<&mut [T]> {
        i.checked_mul(self.line_length)
            .and_then(|start| self.data.get_mut(start..start + self.line_length))
    }

    #[inline]
    pub const fn line_count(&self) -> usize {
        self.line_count
    }

    #[inline]
    pub const fn line_length(&self) -> usize {
        self.line_length
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn iter(&self) -> std::slice::ChunksExact<T> {
        self.data.chunks_exact(self.line_length)
    }

    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::ChunksExactMut<T> {
        self.data.chunks_exact_mut(self.line_length)
    }
}
