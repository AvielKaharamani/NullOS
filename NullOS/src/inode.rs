pub const BLOCKS_IN_INODE: usize = 10;

#[derive(PartialEq)]
pub enum InodeType {
    Free, File, Dir
}

pub struct Inode {
    pub inode_type: InodeType,
    pub content_length: usize,
    pub blocks_indexes: [usize; BLOCKS_IN_INODE]
}