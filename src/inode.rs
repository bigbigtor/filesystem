pub struct Inode {
    inode_type: u8,
    permissions: u8,
    atime: u16,          //access time
    mtime: u16,          //data modification time
    ctime: u16,          //inode modification time
    link_number: u16,    //number of link entries in directory
    log_byte_size: u16,  //size in logic bytes
    oc_data_blocks: u16, //number of ocupied data blocks
    direct_pointers: [u16; 12],
    indirect_pointers: [u16; 3], //[0] simple indirect, [1] double indirect, [2] triple indirect
}

pub const INODE_SIZE: u16 = 64; //bytes
