use mapfs_rpc::rpc;
use fuse::*;
use time::Timespec;
pub trait IsoFuse{
    type F;
    fn to_fuse(&self)->Self::F;
    fn from_fuse(f: &Self::F)->Self;
}

impl IsoFuse for rpc::TimespecRef{
    type F=Timespec;
    fn to_fuse(&self)->Timespec{
        Timespec{
            nsec: self.nsec, sec: self.sec
        }
    }
    fn from_fuse(f: &Timespec)->Self{
        Self{
            nsec: f.nsec, sec: f.sec
        }
    }
}

impl IsoFuse for rpc::FileAttrRef{
    type F=FileAttr;
    fn to_fuse(&self)->FileAttr{
        FileAttr{
            ino: self.ino,
            size: self.size,
            blocks: self.blocks,
            atime: self.atime.to_fuse(),
            mtime: self.mtime.to_fuse(),
            ctime: self.ctime.to_fuse(),
            crtime: self.crtime.to_fuse(),
            kind: self.kind.to_fuse(),
            perm: self.perm,
            nlink: self.nlink,
            uid: self.uid,
            gid: self.gid,
            rdev: self.rdev,
            flags: self.flags,
        }
    }
    fn from_fuse(f: &FileAttr)->Self{
        Self{
            ino: f.ino,
            size: f.size,
            blocks: f.blocks,
            atime: IsoFuse::from_fuse(&f.atime),
            mtime: IsoFuse::from_fuse(&f.mtime),
            ctime: IsoFuse::from_fuse(&f.ctime),
            crtime: IsoFuse::from_fuse(&f.crtime),
            kind: IsoFuse::from_fuse(&f.kind),
            perm: f.perm,
            nlink: f.nlink,
            uid: f.uid,
            gid: f.gid,
            rdev: f.rdev,
            flags: f.flags,
        }
    }
}

impl IsoFuse for rpc::FileTypeRef{
    type F=FileType;
    fn to_fuse(&self)->FileType{
        type A=rpc::FileTypeRef;
        type B=FileType;
        match self{
            A::NamedPipe=>B::NamedPipe,
            A::CharDevice=>B::CharDevice,
            A::BlockDevice=>B::BlockDevice,
            A::Directory=>B::Directory,
            A::RegularFile=>B::RegularFile,
            A::Symlink=>B::Symlink,
            A::Socket=>B::Socket
        }
    }
    fn from_fuse(f: &FileType)->Self{
        type B=rpc::FileTypeRef;
        type A=FileType;
        match f{
            A::NamedPipe=>B::NamedPipe,
            A::CharDevice=>B::CharDevice,
            A::BlockDevice=>B::BlockDevice,
            A::Directory=>B::Directory,
            A::RegularFile=>B::RegularFile,
            A::Symlink=>B::Symlink,
            A::Socket=>B::Socket
        }
    }
}