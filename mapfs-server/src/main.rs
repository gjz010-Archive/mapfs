use tokio::*;
use winapi;
use mapfs_rpc::rpc::*;
use libc::c_int;
use tarpc::{
    client, context,
    server::{self, Handler},
};
#[tokio::main]
async fn main()->std::io::Result<()> {
    println!("Hello, world!");
    Ok(())
}

#[derive(Clone)]
struct FileSystemServer;

#[tarpc::server]
impl FileSystemAsync for FileSystemServer {
    async fn init(self, _: context::Context,req: FRequest) -> Result<(), c_int>{unimplemented!()}
    async fn destroy(self, _: context::Context,req: FRequest){unimplemented!()}
    async fn lookup(self, _: context::Context,req: FRequest, parent: u64, name: String)->FReplyEntry{unimplemented!()}
    async fn forget(self, _: context::Context,req: FRequest, ino: u64, nlookup: u64){unimplemented!()}
    async fn getattr(self, _: context::Context,req: FRequest, ino: u64)->FReplyAttr{unimplemented!()}
    async fn setattr(self, _: context::Context,req: FRequest, attr: FAttrSet)->FReplyAttr{unimplemented!()}
    async fn readlink(self, _: context::Context,req: FRequest, ino: u64)->FReplyData{unimplemented!()}
    async fn mknod(self, _: context::Context,req: FRequest, parent: u64, name: String, mode: u32, rdev: u32)->FReplyEntry{unimplemented!()}
    async fn mkdir(self, _: context::Context,req: FRequest, parent: u64, name: String, mode: u32)->FReplyEntry{unimplemented!()}
    async fn unlink(self, _: context::Context,req: FRequest, parent: u64, name: String)->FReplyEmpty{unimplemented!()}
    async fn rmdir(self, _: context::Context,req: FRequest, parent: u64, name: String)->FReplyEmpty{unimplemented!()}
    async fn symlink(self, _: context::Context,req: FRequest, parent: u64, name: String, link: String)->FReplyEntry{unimplemented!()}
    async fn rename(self, _: context::Context,req: FRequest, parent: u64, name: String, newparent: u64, newname: String)->FReplyEntry{unimplemented!()}
    async fn open(self, _: context::Context,req: FRequest, ino: u64, flags: u32)->FReplyOpen{unimplemented!()}
    async fn link(self, _: context::Context,req: FRequest, ino: u64, newparent: u64, newname: String)->FReplyEntry{unimplemented!()}
    async fn read(self, _: context::Context,req: FRequest, ino: u64, fh: u64, offset: i64, size: u32)->FReplyData{unimplemented!()}
    async fn write(self, _: context::Context,req: FRequest, ino: u64, fh: u64, offset: i64, data: Vec<u8>, flags: u32)->FReplyWrite{unimplemented!()}
    async fn flush(self, _: context::Context,req: FRequest, ino: u64, fh: u64, lock_owner: u64)->FReplyEmpty{unimplemented!()}
    async fn release(self, _: context::Context,req: FRequest, ino: u64, fh: u64, flags: u32, lock_owner: u64, flush: bool)->FReplyEmpty{unimplemented!()}
    async fn fsync(self, _: context::Context,req: FRequest, ino: u64, fh: u64, datasync: bool)->FReplyEmpty{unimplemented!()}
    async fn opendir(self, _: context::Context,req: FRequest, ino: u64, flags: u32)->FReplyOpen{unimplemented!()}
    async fn readdir(self, _: context::Context,req: FRequest, ino: u64, fh: u64, offset: i64, window_size: u64)->FReplyDirectory{unimplemented!()}
    async fn releasedir(self, _: context::Context,req: FRequest, ino: u64, fh:u64, flags: u32)->FReplyEmpty{unimplemented!()}
    async fn fsyncdir(self, _: context::Context,req: FRequest, ino: u64, fh: u64, datasync: bool)->FReplyEmpty{unimplemented!()}
    async fn statfs(self, _: context::Context,req: FRequest, ino: u64)->FReplyStatfs{unimplemented!()}
    async fn setxattr(self, _: context::Context,req: FRequest, ino: u64, name: String, value: Vec<u8>, flags: u32, position: u32)->FReplyEmpty{unimplemented!()}
    async fn getxattr(self, _: context::Context,req: FRequest, ino: u64, name: String, size: u32)->FReplyXattr{unimplemented!()}
    async fn listxattr(self, _: context::Context,req: FRequest, ino: u64, size: u32)->FReplyXattr{unimplemented!()}
    async fn removexattr(self, _: context::Context,req: FRequest, ino: u64, name: String)->FReplyEmpty{unimplemented!()}
    async fn access(self, _: context::Context,req: FRequest, ino: u64, mask: u32)->FReplyEmpty{unimplemented!()}
    async fn create(self, _: context::Context,req: FRequest, parent: u64, name: String, mode: u32, flags: u32)->FReplyCreate{unimplemented!()}
    async fn getlk(self, _: context::Context,req: FRequest, ino: u64, fh: u64, lock_owner: u64, start: u64, end: u64, typ: u32, pid: u32)->FReplyLock{unimplemented!()}
    async fn setlk(self, _: context::Context,req: FRequest, ino: u64, fh: u64, lock_owner: u64, start: u64, end: u64, typ: u32, pid: u32, sleep: bool)->FReplyEmpty{unimplemented!()}
    async fn bmap(self, _: context::Context,req: FRequest, ino: u64, blocksize: u32, idx: u64)->FReplyBmap{unimplemented!()}
}