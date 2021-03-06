use crate::{Point, Rect, Size};
use guillotiere::SimpleAtlasAllocator;

pub struct Allocator {
    raw: SimpleAtlasAllocator,
    allocations: usize,
}

impl Allocator {
    pub fn new(size: u32) -> Allocator {
        let raw = SimpleAtlasAllocator::new(guillotiere::Size::new(
            size as i32,
            size as i32,
        ));

        Allocator {
            raw,
            allocations: 0,
        }
    }

    pub fn allocate(&mut self, width: u32, height: u32) -> Option<Region> {
        let rectangle = self
            .raw
            .allocate(guillotiere::Size::new(width as i32, height as i32))?;

        self.allocations += 1;

        Some(Region {
            area: Rect {
                top_left: Point::new(
                    rectangle.min.x as f32,
                    rectangle.min.y as f32,
                ),
                size: Size::new(width as f32, height as f32),
            },
        })
    }

    /*
    pub fn deallocate(&mut self, region: &Region) {
        self.raw.deallocate(region.allocation.id);

        self.allocations = self.allocations.saturating_sub(1);
    }
    */

    pub fn is_empty(&self) -> bool {
        self.allocations == 0
    }
}

pub struct Region {
    pub area: Rect,
}

impl std::fmt::Debug for Allocator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Allocator")
    }
}

impl std::fmt::Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Region").field("area", &self.area).finish()
    }
}
