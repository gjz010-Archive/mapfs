use libc::c_int;
use serde::*;

#[derive(Debug,Serialize, Deserialize)]
pub enum FileTypeRef {
    NamedPipe,
    CharDevice,
    BlockDevice,
    Directory,
    RegularFile,
    Symlink,
    Socket,
}
#[derive(Debug,Serialize, Deserialize)]
pub struct TimespecRef{
    pub sec: i64,
    pub nsec: i32
}
#[derive(Debug,Serialize, Deserialize)]
pub struct FileAttrRef{
    pub ino: u64,
    pub size: u64,
    pub blocks: u64,
    pub atime: TimespecRef,
    pub mtime: TimespecRef,
    pub ctime: TimespecRef,
    pub crtime: TimespecRef,
    pub kind: FileTypeRef,
    pub perm: u16,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub rdev: u32,
    pub flags: u32,
}

#[derive(Debug,Serialize,Deserialize, Clone, Copy)]
pub struct FRequest{
    pub unique: u64,
    pub uid: u32,
    pub gid: u32,
    pub pid: u32
}
#[derive(Debug,Serialize,Deserialize)]
pub struct FReplyAttrData{
    pub ttl: TimespecRef,
    pub attr: FileAttrRef
}

pub type FReplyAttr = Result<FReplyAttrData, c_int>;
#[derive(Debug,Serialize,Deserialize)]
pub struct FReplyEntryData{
    pub ttl: TimespecRef,
    pub attr: FileAttrRef,
    pub generation: u64
}

pub type FReplyEntry = Result<FReplyEntryData, c_int>;


#[derive(Debug,Serialize,Deserialize)]
pub struct FAttrSet{
    pub ino: u64, 
    pub mode: Option<u32>, 
    pub uid: Option<u32>, 
    pub gid: Option<u32>, 
    pub size: Option<u64>, 
    pub atime: Option<TimespecRef>, 
    pub mtime: Option<TimespecRef>, 
    pub fh: Option<u64>, 
    pub crtime: Option<TimespecRef>, 
    pub chgtime: Option<TimespecRef>, 
    pub bkuptime: Option<TimespecRef>, 
    pub flags: Option<u32>
}


pub type FReplyData = Result<Vec<u8>, c_int>;


pub type FReplyEmpty = Result<(), c_int>;

#[derive(Debug,Serialize,Deserialize)]
pub struct FReplyOpenData {
    pub fh: u64,
    pub flags: u32
}
pub type FReplyOpen = Result<FReplyOpenData, c_int>;

pub type FReplyWrite=Result<u32, c_int>;


#[derive(Debug,Serialize,Deserialize)]
pub struct FReplyDirectoryData{
    pub ino: u64,
    pub offset: i64,
    pub kind: FileTypeRef,
    pub name: String
}

pub type FReplyDirectory=Result<Vec<FReplyDirectoryData>, c_int>;

#[derive(Debug,Serialize,Deserialize)]
pub struct FReplyStatfsData{
    pub blocks: u64, 
    pub bfree: u64, 
    pub bavail: u64, 
    pub files: u64, 
    pub ffree: u64, 
    pub bsize: u32, 
    pub namelen: u32, 
    pub frsize: u32
}
pub type FReplyStatfs=Result<FReplyStatfsData, c_int>;

#[derive(Debug,Serialize,Deserialize)]
pub enum FReplyXattrData{
    Size(u32),
    Data(Vec<u8>)
}
pub type FReplyXattr=Result<FReplyXattrData, c_int>;

#[derive(Debug,Serialize,Deserialize)]
pub struct FReplyCreateData{
    pub ttl: TimespecRef,
    pub attr: FileAttrRef,
    pub generation: u64,
    pub fh: u64,
    pub flags: u32
}
pub type FReplyCreate=Result<FReplyCreateData, c_int>;


#[derive(Debug,Serialize,Deserialize)]
pub struct FReplyLockData{
    pub start: u64, 
    pub end: u64, 
    pub typ: u32, 
    pub pid: u32
}
pub type FReplyLock=Result<FReplyLockData, c_int>;

pub type FReplyBmap=Result<u64, c_int>;

#[tarpc::service]
pub trait FileSystemAsync {
    async fn init(req: FRequest) -> Result<(), c_int>;
    async fn destroy(req: FRequest);
    async fn lookup(req: FRequest, parent: u64, name: String)->FReplyEntry;
    async fn forget(req: FRequest, ino: u64, nlookup: u64);
    async fn getattr(req: FRequest, ino: u64)->FReplyAttr;
    async fn setattr(req: FRequest, attr: FAttrSet)->FReplyAttr;
    async fn readlink(req: FRequest, ino: u64)->FReplyData;
    async fn mknod(req: FRequest, parent: u64, name: String, mode: u32, rdev: u32)->FReplyEntry;
    async fn mkdir(req: FRequest, parent: u64, name: String, mode: u32)->FReplyEntry;
    async fn unlink(req: FRequest, parent: u64, name: String)->FReplyEmpty;
    async fn rmdir(req: FRequest, parent: u64, name: String)->FReplyEmpty;
    async fn symlink(req: FRequest, parent: u64, name: String, link: String)->FReplyEntry;
    async fn rename(req: FRequest, parent: u64, name: String, newparent: u64, newname: String)->FReplyEntry;
    async fn open(req: FRequest, ino: u64, flags: u32)->FReplyOpen;
    async fn link(req: FRequest, ino: u64, newparent: u64, newname: String)->FReplyEntry;
    async fn read(req: FRequest, ino: u64, fh: u64, offset: i64, size: u32)->FReplyData;
    async fn write(req: FRequest, ino: u64, fh: u64, offset: i64, data: Vec<u8>, flags: u32)->FReplyWrite;
    async fn flush(req: FRequest, ino: u64, fh: u64, lock_owner: u64)->FReplyEmpty;
    async fn release(req: FRequest, ino: u64, fh: u64, flags: u32, lock_owner: u64, flush: bool)->FReplyEmpty;
    async fn fsync(req: FRequest, ino: u64, fh: u64, datasync: bool)->FReplyEmpty;
    async fn opendir(req: FRequest, ino: u64, flags: u32)->FReplyOpen;
    async fn readdir(req: FRequest, ino: u64, fh: u64, offset: i64, window_size: u64)->FReplyDirectory;
    async fn releasedir(req: FRequest, ino: u64, fh:u64, flags: u32)->FReplyEmpty;
    async fn fsyncdir(req: FRequest, ino: u64, fh: u64, datasync: bool)->FReplyEmpty;
    async fn statfs(req: FRequest, ino: u64)->FReplyStatfs;
    async fn setxattr(req: FRequest, ino: u64, name: String, value: Vec<u8>, flags: u32, position: u32)->FReplyEmpty;
    async fn getxattr(req: FRequest, ino: u64, name: String, size: u32)->FReplyXattr;
    async fn listxattr(req: FRequest, ino: u64, size: u32)->FReplyXattr;
    async fn removexattr(req: FRequest, ino: u64, name: String)->FReplyEmpty;
    async fn access(req: FRequest, ino: u64, mask: u32)->FReplyEmpty;
    async fn create(req: FRequest, parent: u64, name: String, mode: u32, flags: u32)->FReplyCreate;
    async fn getlk(req: FRequest, ino: u64, fh: u64, lock_owner: u64, start: u64, end: u64, typ: u32, pid: u32)->FReplyLock;
    async fn setlk(req: FRequest, ino: u64, fh: u64, lock_owner: u64, start: u64, end: u64, typ: u32, pid: u32, sleep: bool)->FReplyEmpty;
    async fn bmap(req: FRequest, ino: u64, blocksize: u32, idx: u64)->FReplyBmap;
}

