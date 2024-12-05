use prelude::*;
use std::default::Default;
use std::fmt;
use std::rc::Rc;

pub struct Map {
    _width: usize,
    _height: usize,
    block_size: usize,
    blocks: Rc<[Block]>,
}

impl Map {
    pub unsafe fn draw(&self) {
        for block in self.blocks.iter() {
            DrawRectangleV(
                block.pos,
                Vector2 {
                    x: self.block_size as f32,
                    y: self.block_size as f32,
                },
                block.color,
            );
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pos: Vector2,
    color: Color,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            pos: Vector2 { x: 0.0, y: 0.0 },
            color: colors::BLANK,
        }
    }
}

#[derive(Debug, Default)]
pub struct MapBuilder {
    width: usize,
    height: usize,
    block_size: usize,
    blocks: Vec<Block>,
}

pub enum MapBuilderError {
    WidthModblockSize,
    HeigthModblockSize,
}

impl fmt::Display for MapBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WidthModblockSize => write!(f, "Width must be divisable by block size"),
            Self::HeigthModblockSize => write!(f, "Heigth must be divisable by block size"),
        }
    }
}

impl MapBuilder {
    pub fn new(width: usize, height: usize, block_size: usize) -> Result<Self, MapBuilderError> {
        if width % block_size != 0 {
            return Err(MapBuilderError::WidthModblockSize);
        }

        if height % block_size != 0 {
            return Err(MapBuilderError::HeigthModblockSize);
        }
        let mut blocks: Vec<Block> = Vec::with_capacity(width * height);
        blocks.resize_with(width * height, Default::default);
        blocks.iter_mut().enumerate().for_each(|(i, block)| {
            let x = ((i % width) * block_size) as f32;
            let y = ((i / width) * block_size) as f32;
            block.pos = Vector2 { x, y };
        });
        Ok(MapBuilder {
            width,
            height,
            block_size,
            blocks,
        })
    }
    pub fn set(mut self, x: usize, y: usize, color: Color) -> Self {
        let i = x + self.width * y;
        if let Some(c) = self.blocks.get_mut(i) {
            c.color = color;
        }
        self
    }
    pub fn set_many(mut self, xs: &[(usize, usize, Color)]) -> Self {
        for (x, y, color) in xs {
            let i = x + self.width * y;
            if let Some(c) = self.blocks.get_mut(i) {
                c.color = *color;
            } else {
                return self;
            }
        }
        self
    }
    pub fn fill_vertical(mut self, x: usize, color: Color) -> Self {
        for y in 0..self.height {
            let i = x + self.width * y;
            if let Some(c) = self.blocks.get_mut(i) {
                c.color = color;
            } else {
                return self;
            }
        }
        self
    }
    pub fn fill_horizontal(mut self, y: usize, color: Color) -> Self {
        for x in 0..self.width {
            let i = x + self.width * y;
            if let Some(c) = self.blocks.get_mut(i) {
                c.color = color;
            } else {
                return self;
            }
        }
        self
    }

    pub fn fill_horizontal_strip(
        mut self,
        y: usize,
        start_x: usize,
        end_x: usize,
        color: Color,
    ) -> Self {
        for x in start_x..end_x {
            let i = x + self.width * y;
            if let Some(c) = self.blocks.get_mut(i) {
                c.color = color;
            } else {
                return self;
            }
        }
        self
    }

    pub fn fill_vertical_strip(
        mut self,
        x: usize,
        start_y: usize,
        end_y: usize,
        color: Color,
    ) -> Self {
        for y in start_y..end_y {
            let i = x + self.width * y;
            if let Some(c) = self.blocks.get_mut(i) {
                c.color = color;
            } else {
                return self;
            }
        }
        self
    }

    pub fn apply<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        f(self)
    }
    pub fn build(self) -> Map {
        Map {
            _width: self.width,
            _height: self.height,
            block_size: self.block_size,
            blocks: self.blocks.into(),
        }
    }
}
