use std::sync::atomic::{AtomicUsize, Ordering};

pub struct BlockQueue {
    blocks: Vec<(u32, u32)>,
    dimensions: (u32, u32),
    next: AtomicUsize,
}

impl BlockQueue {
    pub fn new(img: (u32, u32), dim: (u32, u32), select_blocks: (usize, usize)) -> Self {
        if img.0 % dim.0 != 0 || img.1 % dim.1 != 0 {
            panic!(
                "Image with dimension {:?} not evenly divided by blocks of {:?}",
                img, dim
            );
        }
        let num_blocks = (img.0 / dim.0, img.1 / dim.1);
        // TODO: the .. operator precedence is very low so we need this paren here at the moment
        // once (hopefully) it's raised we can remove the parens
        let mut blocks: Vec<(u32, u32)> = (0..num_blocks.0 * num_blocks.1)
            .map(|i| (i % num_blocks.0, i / num_blocks.0))
            .collect();
        blocks.sort_by(|a, b| super::morton2(a).cmp(&super::morton2(b)));
        // If we're only rendering a subset of the blocks then filter our list down
        if select_blocks.1 > 0 {
            blocks = blocks
                .into_iter()
                .skip(select_blocks.0)
                .take(select_blocks.1)
                .collect();
        }
        if blocks.is_empty() {
            println!("Warning: This block queue is empty!");
        }

        Self {
            blocks,
            dimensions: dim,
            next: AtomicUsize::new(0),
        }
    }

    pub fn block_dim(&self) -> (u32, u32) {
        self.dimensions
    }

    pub fn iter(&self) -> BlockQueueIterator {
        BlockQueueIterator { queue: self }
    }

    fn next(&self) -> Option<(u32, u32)> {
        let i = self.next.fetch_add(1, Ordering::AcqRel);
        if i >= self.blocks.len() {
            None
        } else {
            Some(self.blocks[i])
        }
    }
}

pub struct BlockQueueIterator<'a> {
    queue: &'a BlockQueue,
}

impl<'a> Iterator for BlockQueueIterator<'a> {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<(u32, u32)> {
        self.queue.next()
    }
}
