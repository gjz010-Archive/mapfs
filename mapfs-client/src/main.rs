use std::env;
use std::ffi::{OsStr};
use libc::c_int;
use time::Timespec;
use mapfs_rpc::rpc::*;
mod convert;
use fuse::*;
use std::path::Path;

use tokio::sync::Mutex;

use tokio_serde::formats::Bincode;
use std::{io, net::SocketAddr};

use std::sync::Arc;
use tarpc::{
    client, context
};
use crate::convert::*;

pub struct RemoteFS{client: Arc<Mutex<FileSystemAsyncClient>>}

fn request_f(req: &Request)->FRequest{
    FRequest{
        unique: req.unique(),
        uid: req.uid(),
        pid: req.pid(),
        gid: req.gid()
    }
}
impl Filesystem for RemoteFS{
    fn init(&mut self, req: &Request) -> Result<(), c_int> {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            client.init(context::current(), f_req).await.unwrap()
            
        });
        Ok(())
    }
    fn destroy(&mut self, req: &Request) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            client.destroy(context::current(), f_req).await.unwrap()
        });
        
    }
    fn lookup(
        &mut self, 
        req: &Request, 
        parent: u64, 
        name: &OsStr, 
        reply: ReplyEntry
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        let name=String::from(name.to_str().unwrap());
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.lookup(context::current(), f_req, parent, name).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.entry(&data.ttl.to_fuse(), &data.attr.to_fuse(), data.generation);
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn forget(&mut self, req: &Request, ino: u64, nlookup: u64) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            client.forget(context::current(), f_req, ino, nlookup).await.unwrap();
        });
    }
    fn getattr(&mut self, req: &Request, ino: u64, reply: ReplyAttr) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.getattr(context::current(), f_req, ino).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.attr(&data.ttl.to_fuse(), &data.attr.to_fuse())
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn setattr(
        &mut self, 
        req: &Request, 
        ino: u64, 
        mode: Option<u32>, 
        uid: Option<u32>, 
        gid: Option<u32>, 
        size: Option<u64>, 
        atime: Option<Timespec>, 
        mtime: Option<Timespec>, 
        fh: Option<u64>, 
        crtime: Option<Timespec>, 
        chgtime: Option<Timespec>, 
        bkuptime: Option<Timespec>, 
        flags: Option<u32>, 
        reply: ReplyAttr
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.setattr(context::current(), f_req, FAttrSet{
                ino,mode,uid,gid,size,fh,flags,
                atime: atime.map(|x| {IsoFuse::from_fuse(&x)}),
                mtime: mtime.map(|x| {IsoFuse::from_fuse(&x)}),
                crtime: crtime.map(|x| {IsoFuse::from_fuse(&x)}),
                chgtime: chgtime.map(|x| {IsoFuse::from_fuse(&x)}),
                bkuptime: bkuptime.map(|x| {IsoFuse::from_fuse(&x)})
            }).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.attr(&data.ttl.to_fuse(), &data.attr.to_fuse());
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });

    }
    fn readlink(&mut self, req: &Request, ino: u64, reply: ReplyData) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.readlink(context::current(), f_req, ino).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.data(&data)
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn mknod(
        &mut self, 
        req: &Request, 
        parent: u64, 
        name: &OsStr, 
        mode: u32, 
        rdev: u32, 
        reply: ReplyEntry
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        let name=String::from(name.to_str().unwrap());
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.mknod(context::current(), f_req, parent, name, mode, rdev).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.entry(&data.ttl.to_fuse(), &data.attr.to_fuse(), data.generation)
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn mkdir(
        &mut self, 
        req: &Request, 
        parent: u64, 
        name: &OsStr, 
        mode: u32, 
        reply: ReplyEntry
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        let name=String::from(name.to_str().unwrap());
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.mkdir(context::current(), f_req, parent, name, mode).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.entry(&data.ttl.to_fuse(), &data.attr.to_fuse(), data.generation)
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn unlink(
        &mut self, 
        req: &Request, 
        parent: u64, 
        name: &OsStr, 
        reply: ReplyEmpty
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        let name=String::from(name.to_str().unwrap());
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.unlink(context::current(), f_req, parent, name).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.ok()
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn rmdir(
        &mut self, 
        req: &Request, 
        parent: u64, 
        name: &OsStr, 
        reply: ReplyEmpty
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        let name=String::from(name.to_str().unwrap());
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.rmdir(context::current(), f_req, parent, name).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.ok()
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        }); 
    }
    fn symlink(
        &mut self, 
        req: &Request, 
        parent: u64, 
        name: &OsStr, 
        link: &Path, 
        reply: ReplyEntry
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        let name=String::from(name.to_str().unwrap());
        let link=String::from(link.to_str().unwrap());
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.symlink(context::current(), f_req, parent, name, link).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.entry(&data.ttl.to_fuse(), &data.attr.to_fuse(), data.generation)
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn rename(
        &mut self, 
        req: &Request, 
        parent: u64, 
        name: &OsStr, 
        newparent: u64, 
        newname: &OsStr, 
        reply: ReplyEmpty
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        let name=String::from(name.to_str().unwrap());
        let newname=String::from(newname.to_str().unwrap());
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.rename(context::current(), f_req, parent, name, newparent, newname).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.ok()
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn link(
        &mut self, 
        req: &Request, 
        ino: u64, 
        newparent: u64, 
        newname: &OsStr, 
        reply: ReplyEntry
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        let newname=String::from(newname.to_str().unwrap());
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.link(context::current(), f_req, ino, newparent, newname).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.entry(&data.ttl.to_fuse(), &data.attr.to_fuse(), data.generation)
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn open(&mut self, req: &Request, ino: u64, flags: u32, reply: ReplyOpen) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.open(context::current(), f_req, ino, flags).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.opened(data.fh, data.flags)
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn read(
        &mut self, 
        req: &Request, 
        ino: u64, 
        fh: u64, 
        offset: i64, 
        size: u32, 
        reply: ReplyData
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.read(context::current(), f_req, ino, fh, offset, size).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.data(&data)
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn write(
        &mut self, 
        req: &Request, 
        ino: u64, 
        fh: u64, 
        offset: i64, 
        data: &[u8], 
        flags: u32, 
        reply: ReplyWrite
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        let data=Vec::from(data);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.write(context::current(), f_req, ino, fh, offset, data, flags).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.written(data)
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn flush(
        &mut self, 
        req: &Request, 
        ino: u64, 
        fh: u64, 
        lock_owner: u64, 
        reply: ReplyEmpty
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.flush(context::current(), f_req, ino, fh, lock_owner).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.ok()
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn release(
        &mut self, 
        req: &Request, 
        ino: u64, 
        fh: u64, 
        flags: u32, 
        lock_owner: u64, 
        flush: bool, 
        reply: ReplyEmpty
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.release(context::current(), f_req, ino, fh, flags, lock_owner, flush).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.ok()
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn fsync(
        &mut self, 
        req: &Request, 
        ino: u64, 
        fh: u64, 
        datasync: bool, 
        reply: ReplyEmpty
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.fsync(context::current(), f_req, ino, fh, datasync).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.ok()
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn opendir(
        &mut self, 
        req: &Request, 
        ino: u64, 
        flags: u32, 
        reply: ReplyOpen
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.opendir(context::current(), f_req, ino, flags).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.opened(data.fh, data.flags)
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn readdir(
        &mut self, 
        req: &Request, 
        ino: u64, 
        fh: u64, 
        offset: i64, 
        mut reply: ReplyDirectory
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let mut offset=offset;
            let mut buffer_size=4;
            'readdir:loop{
                let ret=client.readdir(context::current(), f_req, ino, fh, offset, buffer_size).await.unwrap();
                match ret{
                    Ok(data)=>{
                        if data.len()==0{
                            reply.ok();
                            break 'readdir;
                        }else{
                            offset+=data.len() as i64;
                            for item in data{
                                if reply.add(item.ino, item.offset, item.kind.to_fuse(), OsStr::new(&item.name)){
                                    // buffer full.
                                    reply.ok();
                                    break 'readdir;
                                }
                            }
                            
                            if buffer_size<128{
                                buffer_size*=2;
                            }
                        }
                    }
                    Err(err)=>{
                        reply.error(err);
                        break 'readdir;
                    }
                }
            }
        });

    }
    fn releasedir(
        &mut self, 
        req: &Request, 
        ino: u64, 
        fh: u64, 
        flags: u32, 
        reply: ReplyEmpty
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.releasedir(context::current(), f_req, ino, fh, flags).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.ok()
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn fsyncdir(
        &mut self, 
        req: &Request, 
        ino: u64, 
        fh: u64, 
        datasync: bool, 
        reply: ReplyEmpty
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.fsyncdir(context::current(), f_req, ino, fh, datasync).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.ok()
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn statfs(&mut self, req: &Request, ino: u64, reply: ReplyStatfs) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.statfs(context::current(), f_req, ino).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.statfs(data.blocks, data.bfree, data.bavail, data.files, data.ffree, data.bsize, data.namelen, data.frsize)
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn setxattr(
        &mut self, 
        req: &Request, 
        ino: u64, 
        name: &OsStr, 
        value: &[u8], 
        flags: u32, 
        position: u32, 
        reply: ReplyEmpty
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        let name=String::from(name.to_str().unwrap());
        let value=Vec::from(value);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.setxattr(context::current(), f_req, ino, name, value, flags, position).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.ok()
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn getxattr(
        &mut self, 
        req: &Request, 
        ino: u64, 
        name: &OsStr, 
        size: u32, 
        reply: ReplyXattr
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        let name=String::from(name.to_str().unwrap());
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.getxattr(context::current(), f_req, ino, name, size).await.unwrap();
            match ret{
                Ok(FReplyXattrData::Data(x))=>{
                    reply.data(&x)
                }
                Ok(FReplyXattrData::Size(x))=>{
                    reply.size(x)
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn listxattr(
        &mut self, 
        req: &Request, 
        ino: u64, 
        size: u32, 
        reply: ReplyXattr
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.listxattr(context::current(), f_req, ino, size).await.unwrap();
            match ret{
                Ok(FReplyXattrData::Data(x))=>{
                    reply.data(&x)
                }
                Ok(FReplyXattrData::Size(x))=>{
                    reply.size(x)
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn removexattr(
        &mut self, 
        req: &Request, 
        ino: u64, 
        name: &OsStr, 
        reply: ReplyEmpty
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        let name=String::from(name.to_str().unwrap());
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.removexattr(context::current(), f_req, ino, name).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.ok()
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn access(
        &mut self, 
        req: &Request, 
        ino: u64, 
        mask: u32, 
        reply: ReplyEmpty
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.access(context::current(), f_req, ino, mask).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.ok()
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn create(
        &mut self, 
        req: &Request, 
        parent: u64, 
        name: &OsStr, 
        mode: u32, 
        flags: u32, 
        reply: ReplyCreate
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        let name=String::from(name.to_str().unwrap());
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.create(context::current(), f_req, parent, name, mode, flags).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.created(&data.ttl.to_fuse(), &data.attr.to_fuse(), data.generation, data.fh, data.flags)
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn getlk(
        &mut self, 
        req: &Request, 
        ino: u64, 
        fh: u64, 
        lock_owner: u64, 
        start: u64, 
        end: u64, 
        typ: u32, 
        pid: u32, 
        reply: ReplyLock
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.getlk(context::current(), f_req, ino, fh, lock_owner, start, end, typ, pid).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.locked(data.start, data.end, data.typ, data.pid)
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn setlk(
        &mut self, 
        req: &Request, 
        ino: u64, 
        fh: u64, 
        lock_owner: u64, 
        start: u64, 
        end: u64, 
        typ: u32, 
        pid: u32, 
        sleep: bool, 
        reply: ReplyEmpty
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.setlk(context::current(), f_req, ino, fh, lock_owner, start, end, typ, pid, sleep).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.ok()
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
    fn bmap(
        &mut self, 
        req: &Request, 
        ino: u64, 
        blocksize: u32, 
        idx: u64, 
        reply: ReplyBmap
    ) {
        let client=Arc::clone(&self.client);
        let f_req=request_f(req);
        tokio::spawn(async move{
            let mut client=client.lock().await;
            let ret=client.bmap(context::current(), f_req, ino, blocksize, idx).await.unwrap();
            match ret{
                Ok(data)=>{
                    reply.bmap(data)
                }
                Err(err)=>{
                    reply.error(err)
                }
            }
        });
    }
}


#[tokio::main]
async fn main() -> io::Result<()>{
    env_logger::init();
    let mut args=env::args();
    let mountpoint=args.nth(1).unwrap();
    let server_addr=args.nth(0).unwrap();
    let server_addr = server_addr.parse::<SocketAddr>().unwrap();
    let mut stream = tarpc::serde_transport::tcp::connect(server_addr, Bincode::default()).await?;
    let tarpc::rpc::client::NewClient {client, dispatch}=FileSystemAsyncClient::new(client::Config::default(), stream);
    let rfs=RemoteFS{client: Arc::new(Mutex::new(client))};

    let session=unsafe {
        fuse::spawn_mount(rfs, &OsStr::new(&mountpoint), &[   
        OsStr::new("-o"), OsStr::new("default_permissions"), 
        OsStr::new("-o"), OsStr::new("fsname=mappedfilesystem"), 
        OsStr::new("-o"), OsStr::new("subtype=winmapfs"),
        OsStr::new("-o"), OsStr::new("auto_unmount"),
        ])
    }.unwrap();
    tokio::spawn(async move {
        let sess=session; // move in
        if let Err(e)=dispatch.await{
            panic!("Error while running client dispatch: {:?}", e)
        }
    });
    Ok(())
}