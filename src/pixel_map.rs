use crate::{geometry::Point2, FromProto};
use ndarray::Array2;
use num_traits::FromPrimitive;
use sc2_proto::common::ImageData;
use std::ops::{Index, IndexMut};

pub type PixelMap = Array2<Pixel>;
pub type ByteMap = Array2<u8>;
pub type VisibilityMap = Array2<Visibility>;

impl<T> Index<Point2> for Array2<T> {
	type Output = T;

	#[inline]
	fn index(&self, pos: Point2) -> &Self::Output {
		&self[((pos.x + 0.5) as usize, (pos.y + 0.5) as usize)]
	}
}
impl<T> IndexMut<Point2> for Array2<T> {
	#[inline]
	fn index_mut(&mut self, pos: Point2) -> &mut Self::Output {
		&mut self[((pos.x + 0.5) as usize, (pos.y + 0.5) as usize)]
	}
}

fn to_binary(n: u8) -> Vec<Pixel> {
	match n {
		0 => vec![Pixel::Set; 8],
		255 => vec![Pixel::Empty; 8],
		_ => (0..8)
			.rev()
			.map(|x| Pixel::from_u8((n >> x) & 1).unwrap())
			.collect(),
	}
}

impl FromProto<ImageData> for PixelMap {
	fn from_proto(grid: ImageData) -> Self {
		let size = grid.get_size();
		Array2::from_shape_vec(
			(size.get_y() as usize, size.get_x() as usize),
			grid.get_data().iter().flat_map(|n| to_binary(*n)).collect(),
		)
		.expect("Can't create PixelMap")
		.reversed_axes()
	}
}
impl FromProto<ImageData> for ByteMap {
	fn from_proto(grid: ImageData) -> Self {
		let size = grid.get_size();
		Array2::from_shape_vec(
			(size.get_y() as usize, size.get_x() as usize),
			grid.get_data().iter().copied().collect(),
		)
		.expect("Can't create ByteMap")
		.reversed_axes()
	}
}
impl FromProto<ImageData> for VisibilityMap {
	fn from_proto(grid: ImageData) -> Self {
		let size = grid.get_size();
		Array2::from_shape_vec(
			(size.get_y() as usize, size.get_x() as usize),
			grid.get_data()
				.iter()
				.map(|n| {
					Visibility::from_u8(*n)
						.unwrap_or_else(|| panic!("enum Visibility has no variant with value: {}", n))
				})
				.collect(),
		)
		.expect("Can't create VisibilityMap")
		.reversed_axes()
	}
}

#[derive(FromPrimitive, ToPrimitive, Copy, Clone, PartialEq, Eq)]
pub enum Pixel {
	Set,
	Empty,
}
impl Pixel {
	pub fn is_empty(self) -> bool {
		matches!(self, Pixel::Empty)
	}
	pub fn is_set(self) -> bool {
		matches!(self, Pixel::Set)
	}
}
impl Default for Pixel {
	fn default() -> Self {
		Pixel::Empty
	}
}
impl std::fmt::Debug for Pixel {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Pixel::Empty => 0.fmt(f),
			Pixel::Set => 1.fmt(f),
		}
	}
}

#[derive(Debug, FromPrimitive, ToPrimitive, Copy, Clone, PartialEq, Eq)]
pub enum Visibility {
	Hidden,
	Fogged,
	Visible,
	FullHidden,
}
impl Visibility {
	pub fn is_hidden(self) -> bool {
		matches!(self, Visibility::Hidden)
	}
	pub fn is_fogged(self) -> bool {
		matches!(self, Visibility::Fogged)
	}
	pub fn is_visible(self) -> bool {
		matches!(self, Visibility::Visible)
	}
	pub fn is_full_hidden(self) -> bool {
		matches!(self, Visibility::FullHidden)
	}
	pub fn is_explored(self) -> bool {
		!matches!(self, Visibility::Hidden)
	}
}
impl Default for Visibility {
	fn default() -> Self {
		Visibility::Hidden
	}
}
