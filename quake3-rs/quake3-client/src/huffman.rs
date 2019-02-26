#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(custom_attribute, libc)]
extern crate libc;
#[header_src =
      "ioq3/code/qcommon/q_shared.h"]
pub mod q_shared_h {
    /*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
//
    // q_shared.h -- included first by ALL program modules.
// A user mod should never modify this file
    // Heartbeat for dpmaster protocol. You shouldn't change this unless you know what you're doing
    // When com_gamename is LEGACY_MASTER_GAMENAME, use quake3 master protocol.
// You shouldn't change this unless you know what you're doing
    // number of supported master servers
    // standard demo extension
    //Ignore __attribute__ on non-gcc platforms
    /* *********************************************************************
  VM Considerations

  The VM can not use the standard system headers because we aren't really
  using the compiler they were meant for.  We use bg_lib.h which contains
  prototypes for the functions we define for our own use in bg_lib.c.

  When writing mods, please add needed headers HERE, do not start including
  stuff like <stdio.h> in the various .c files that make up each of the VMs
  since you will be including system headers files can will have issues.

  Remember, if you use a C library function that is not defined in bg_lib.c,
  you will have to add your own version for support in the VM.

 **********************************************************************/
    //=============================================================
    pub type byte = libc::c_uchar;
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    use super::{libc};
}
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    /*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
// qcommon.h -- definitions common between client and server, but not game.or ref modules
    //Ignore __attribute__ on non-gcc platforms
    //#define	PRE_RELEASE_DEMO
    //============================================================================
    //
// msg.c
//
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct msg_t {
        pub allowoverflow: qboolean,
        pub overflowed: qboolean,
        pub oob: qboolean,
        pub data: *mut byte,
        pub maxsize: libc::c_int,
        pub cursize: libc::c_int,
        pub readcount: libc::c_int,
        pub bit: libc::c_int,
    }
    /* This is based on the Adaptive Huffman algorithm described in Sayood's Data
 * Compression book.  The ranks are not actually stored, but implicitly defined
 * by the location of a node within a doubly-linked list */
    /* NYT = Not Yet Transmitted */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct nodetype {
        pub left: *mut nodetype,
        pub right: *mut nodetype,
        pub parent: *mut nodetype,
        pub next: *mut nodetype,
        pub prev: *mut nodetype,
        pub head: *mut *mut nodetype,
        pub weight: libc::c_int,
        pub symbol: libc::c_int,
    }
    pub type node_t = nodetype;
    /* Maximum symbol */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct huff_t {
        pub blocNode: libc::c_int,
        pub blocPtrs: libc::c_int,
        pub tree: *mut node_t,
        pub lhead: *mut node_t,
        pub ltail: *mut node_t,
        pub loc: [*mut node_t; 257],
        pub freelist: *mut *mut node_t,
        pub nodeList: [node_t; 768],
        pub nodePtrs: [*mut node_t; 768],
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct huffman_t {
        pub compressor: huff_t,
        pub decompressor: huff_t,
    }
    use super::q_shared_h::{qboolean, byte};
    use super::{libc};
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[header_src =
      "ioq3/code/qcommon/huffman.c"]
pub mod huffman_c { }
use self::q_shared_h::{byte, qboolean, qtrue, qfalse};
use self::qcommon_h::{msg_t, nodetype, node_t, huff_t, huffman_t};
use self::string_h::{memcpy, memset};
#[no_mangle]
pub unsafe extern "C" fn Huff_Compress(mut mbuf: *mut msg_t,
                                       mut offset: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut seq: [byte; 65536] = [0; 65536];
    let mut buffer: *mut byte = 0 as *mut byte;
    let mut huff: huff_t =
        huff_t{blocNode: 0,
               blocPtrs: 0,
               tree: 0 as *mut node_t,
               lhead: 0 as *mut node_t,
               ltail: 0 as *mut node_t,
               loc: [0 as *mut node_t; 257],
               freelist: 0 as *mut *mut node_t,
               nodeList:
                   [nodetype{left: 0 as *mut nodetype,
                             right: 0 as *mut nodetype,
                             parent: 0 as *mut nodetype,
                             next: 0 as *mut nodetype,
                             prev: 0 as *mut nodetype,
                             head: 0 as *mut *mut nodetype,
                             weight: 0,
                             symbol: 0,}; 768],
               nodePtrs: [0 as *mut node_t; 768],};
    size = (*mbuf).cursize - offset;
    buffer = (*mbuf).data.offset(offset as isize);
    if size <= 0i32 { return }
    memset(&mut huff as *mut huff_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<huff_t>() as libc::c_ulong);
    let fresh0 = huff.blocNode;
    huff.blocNode = huff.blocNode + 1;
    huff.loc[256usize] =
        &mut *huff.nodeList.as_mut_ptr().offset(fresh0 as isize) as
            *mut node_t;
    huff.lhead = huff.loc[256usize];
    huff.tree = huff.lhead;
    (*huff.tree).symbol = 256i32;
    (*huff.tree).weight = 0i32;
    (*huff.lhead).prev = 0 as *mut nodetype;
    (*huff.lhead).next = (*huff.lhead).prev;
    (*huff.tree).right = 0 as *mut nodetype;
    (*huff.tree).left = (*huff.tree).right;
    (*huff.tree).parent = (*huff.tree).left;
    seq[0usize] = (size >> 8i32) as byte;
    seq[1usize] = (size & 0xffi32) as byte;
    bloc = 16i32;
    i = 0i32;
    while i < size {
        ch = *buffer.offset(i as isize) as libc::c_int;
        Huff_transmit(&mut huff, ch, seq.as_mut_ptr(), size << 3i32);
        Huff_addRef(&mut huff, ch as byte);
        i += 1
    }
    bloc += 8i32;
    (*mbuf).cursize = (bloc >> 3i32) + offset;
    memcpy((*mbuf).data.offset(offset as isize) as *mut libc::c_void,
           seq.as_mut_ptr() as *const libc::c_void,
           (bloc >> 3i32) as libc::c_ulong);
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
/* This is based on the Adaptive Huffman algorithm described in Sayood's Data
 * Compression book.  The ranks are not actually stored, but implicitly defined
 * by the location of a node within a doubly-linked list */
static mut bloc: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn Huff_addRef(mut huff: *mut huff_t, mut ch: byte) {
    let mut tnode: *mut node_t = 0 as *mut node_t;
    let mut tnode2: *mut node_t = 0 as *mut node_t;
    if (*huff).loc[ch as usize].is_null() {
        let fresh1 = (*huff).blocNode;
        (*huff).blocNode = (*huff).blocNode + 1;
        tnode =
            &mut *(*huff).nodeList.as_mut_ptr().offset(fresh1 as isize) as
                *mut node_t;
        let fresh2 = (*huff).blocNode;
        (*huff).blocNode = (*huff).blocNode + 1;
        tnode2 =
            &mut *(*huff).nodeList.as_mut_ptr().offset(fresh2 as isize) as
                *mut node_t;
        (*tnode2).symbol = 256i32 + 1i32;
        (*tnode2).weight = 1i32;
        (*tnode2).next = (*(*huff).lhead).next;
        if !(*(*huff).lhead).next.is_null() {
            (*(*(*huff).lhead).next).prev = tnode2;
            if (*(*(*huff).lhead).next).weight == 1i32 {
                (*tnode2).head = (*(*(*huff).lhead).next).head
            } else {
                (*tnode2).head = get_ppnode(huff);
                *(*tnode2).head = tnode2
            }
        } else { (*tnode2).head = get_ppnode(huff); *(*tnode2).head = tnode2 }
        (*(*huff).lhead).next = tnode2;
        (*tnode2).prev = (*huff).lhead;
        (*tnode).symbol = ch as libc::c_int;
        (*tnode).weight = 1i32;
        (*tnode).next = (*(*huff).lhead).next;
        if !(*(*huff).lhead).next.is_null() {
            (*(*(*huff).lhead).next).prev = tnode;
            if (*(*(*huff).lhead).next).weight == 1i32 {
                (*tnode).head = (*(*(*huff).lhead).next).head
            } else {
                (*tnode).head = get_ppnode(huff);
                *(*tnode).head = tnode2
            }
        } else { (*tnode).head = get_ppnode(huff); *(*tnode).head = tnode }
        (*(*huff).lhead).next = tnode;
        (*tnode).prev = (*huff).lhead;
        (*tnode).right = 0 as *mut nodetype;
        (*tnode).left = (*tnode).right;
        if !(*(*huff).lhead).parent.is_null() {
            if (*(*(*huff).lhead).parent).left == (*huff).lhead {
                (*(*(*huff).lhead).parent).left = tnode2
            } else { (*(*(*huff).lhead).parent).right = tnode2 }
        } else { (*huff).tree = tnode2 }
        (*tnode2).right = tnode;
        (*tnode2).left = (*huff).lhead;
        (*tnode2).parent = (*(*huff).lhead).parent;
        (*tnode).parent = tnode2;
        (*(*huff).lhead).parent = (*tnode).parent;
        (*huff).loc[ch as usize] = tnode;
        increment(huff, (*tnode2).parent);
    } else { increment(huff, (*huff).loc[ch as usize]); };
}
/* Do the increments */
unsafe extern "C" fn increment(mut huff: *mut huff_t, mut node: *mut node_t) {
    let mut lnode: *mut node_t = 0 as *mut node_t;
    if node.is_null() { return }
    if !(*node).next.is_null() && (*(*node).next).weight == (*node).weight {
        lnode = *(*node).head;
        if lnode != (*node).parent { swap(huff, lnode, node); }
        swaplist(lnode, node);
    }
    if !(*node).prev.is_null() && (*(*node).prev).weight == (*node).weight {
        *(*node).head = (*node).prev
    } else {
        *(*node).head = 0 as *mut nodetype;
        free_ppnode(huff, (*node).head);
    }
    (*node).weight += 1;
    if !(*node).next.is_null() && (*(*node).next).weight == (*node).weight {
        (*node).head = (*(*node).next).head
    } else { (*node).head = get_ppnode(huff); *(*node).head = node }
    if !(*node).parent.is_null() {
        increment(huff, (*node).parent);
        if (*node).prev == (*node).parent {
            swaplist(node, (*node).parent);
            if *(*node).head == node { *(*node).head = (*node).parent }
        }
    };
}
/* Swap these two nodes in the linked list (update ranks) */
unsafe extern "C" fn swaplist(mut node1: *mut node_t,
                              mut node2: *mut node_t) {
    let mut par1: *mut node_t = 0 as *mut node_t;
    par1 = (*node1).next;
    (*node1).next = (*node2).next;
    (*node2).next = par1;
    par1 = (*node1).prev;
    (*node1).prev = (*node2).prev;
    (*node2).prev = par1;
    if (*node1).next == node1 { (*node1).next = node2 }
    if (*node2).next == node2 { (*node2).next = node1 }
    if !(*node1).next.is_null() { (*(*node1).next).prev = node1 }
    if !(*node2).next.is_null() { (*(*node2).next).prev = node2 }
    if !(*node1).prev.is_null() { (*(*node1).prev).next = node1 }
    if !(*node2).prev.is_null() { (*(*node2).prev).next = node2 };
}
unsafe extern "C" fn get_ppnode(mut huff: *mut huff_t) -> *mut *mut node_t {
    let mut tppnode: *mut *mut node_t = 0 as *mut *mut node_t;
    if (*huff).freelist.is_null() {
        let fresh3 = (*huff).blocPtrs;
        (*huff).blocPtrs = (*huff).blocPtrs + 1;
        return &mut *(*huff).nodePtrs.as_mut_ptr().offset(fresh3 as isize) as
                   *mut *mut node_t
    } else {
        tppnode = (*huff).freelist;
        (*huff).freelist = *tppnode as *mut *mut node_t;
        return tppnode
    };
}
unsafe extern "C" fn free_ppnode(mut huff: *mut huff_t,
                                 mut ppnode: *mut *mut node_t) {
    *ppnode = (*huff).freelist as *mut node_t;
    (*huff).freelist = ppnode;
}
/* Swap the location of these two nodes in the tree */
unsafe extern "C" fn swap(mut huff: *mut huff_t, mut node1: *mut node_t,
                          mut node2: *mut node_t) {
    let mut par1: *mut node_t = 0 as *mut node_t;
    let mut par2: *mut node_t = 0 as *mut node_t;
    par1 = (*node1).parent;
    par2 = (*node2).parent;
    if !par1.is_null() {
        if (*par1).left == node1 {
            (*par1).left = node2
        } else { (*par1).right = node2 }
    } else { (*huff).tree = node2 }
    if !par2.is_null() {
        if (*par2).left == node2 {
            (*par2).left = node1
        } else { (*par2).right = node1 }
    } else { (*huff).tree = node1 }
    (*node1).parent = par2;
    (*node2).parent = par1;
}
#[no_mangle]
pub unsafe extern "C" fn Huff_transmit(mut huff: *mut huff_t,
                                       mut ch: libc::c_int,
                                       mut fout: *mut byte,
                                       mut maxoffset: libc::c_int) {
    let mut i: libc::c_int = 0;
    if (*huff).loc[ch as usize].is_null() {
        Huff_transmit(huff, 256i32, fout, maxoffset);
        i = 7i32;
        while i >= 0i32 {
            add_bit((ch >> i & 0x1i32) as libc::c_char, fout);
            i -= 1
        }
    } else {
        send((*huff).loc[ch as usize], 0 as *mut node_t, fout, maxoffset);
    };
}
/* Send the prefix code for this node */
unsafe extern "C" fn send(mut node: *mut node_t, mut child: *mut node_t,
                          mut fout: *mut byte, mut maxoffset: libc::c_int) {
    if !(*node).parent.is_null() {
        send((*node).parent, node, fout, maxoffset);
    }
    if !child.is_null() {
        if bloc >= maxoffset { bloc = maxoffset + 1i32; return }
        if (*node).right == child {
            add_bit(1i32 as libc::c_char, fout);
        } else { add_bit(0i32 as libc::c_char, fout); }
    };
}
/* Add a bit to the output file (buffered) */
unsafe extern "C" fn add_bit(mut bit: libc::c_char, mut fout: *mut byte) {
    if bloc & 7i32 == 0i32 {
        *fout.offset((bloc >> 3i32) as isize) = 0i32 as byte
    }
    let ref mut fresh4 = *fout.offset((bloc >> 3i32) as isize);
    *fresh4 =
        (*fresh4 as libc::c_int | (bit as libc::c_int) << (bloc & 7i32)) as
            byte;
    bloc += 1;
}
#[no_mangle]
pub unsafe extern "C" fn Huff_Decompress(mut mbuf: *mut msg_t,
                                         mut offset: libc::c_int) {
    let mut ch: libc::c_int = 0;
    let mut cch: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut seq: [byte; 65536] = [0; 65536];
    let mut buffer: *mut byte = 0 as *mut byte;
    let mut huff: huff_t =
        huff_t{blocNode: 0,
               blocPtrs: 0,
               tree: 0 as *mut node_t,
               lhead: 0 as *mut node_t,
               ltail: 0 as *mut node_t,
               loc: [0 as *mut node_t; 257],
               freelist: 0 as *mut *mut node_t,
               nodeList:
                   [nodetype{left: 0 as *mut nodetype,
                             right: 0 as *mut nodetype,
                             parent: 0 as *mut nodetype,
                             next: 0 as *mut nodetype,
                             prev: 0 as *mut nodetype,
                             head: 0 as *mut *mut nodetype,
                             weight: 0,
                             symbol: 0,}; 768],
               nodePtrs: [0 as *mut node_t; 768],};
    size = (*mbuf).cursize - offset;
    buffer = (*mbuf).data.offset(offset as isize);
    if size <= 0i32 { return }
    memset(&mut huff as *mut huff_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<huff_t>() as libc::c_ulong);
    let fresh5 = huff.blocNode;
    huff.blocNode = huff.blocNode + 1;
    huff.loc[256usize] =
        &mut *huff.nodeList.as_mut_ptr().offset(fresh5 as isize) as
            *mut node_t;
    huff.ltail = huff.loc[256usize];
    huff.lhead = huff.ltail;
    huff.tree = huff.lhead;
    (*huff.tree).symbol = 256i32;
    (*huff.tree).weight = 0i32;
    (*huff.lhead).prev = 0 as *mut nodetype;
    (*huff.lhead).next = (*huff.lhead).prev;
    (*huff.tree).right = 0 as *mut nodetype;
    (*huff.tree).left = (*huff.tree).right;
    (*huff.tree).parent = (*huff.tree).left;
    cch =
        *buffer.offset(0isize) as libc::c_int * 256i32 +
            *buffer.offset(1isize) as libc::c_int;
    if cch > (*mbuf).maxsize - offset { cch = (*mbuf).maxsize - offset }
    bloc = 16i32;
    j = 0i32;
    while j < cch {
        ch = 0i32;
        // don't overflow reading from the messages
		// FIXME: would it be better to have an overflow check in get_bit ?
        if bloc >> 3i32 > size {
            seq[j as usize] = 0i32 as byte;
            break ;
        } else {
            Huff_Receive(huff.tree, &mut ch, buffer);
            if ch == 256i32 {
                ch = 0i32;
                i = 0i32;
                while i < 8i32 { ch = (ch << 1i32) + get_bit(buffer); i += 1 }
            }
            seq[j as usize] = ch as byte;
            Huff_addRef(&mut huff, ch as byte);
            j += 1
        }
    }
    (*mbuf).cursize = cch + offset;
    memcpy((*mbuf).data.offset(offset as isize) as *mut libc::c_void,
           seq.as_mut_ptr() as *const libc::c_void, cch as libc::c_ulong);
}
/* Receive one bit from the input file (buffered) */
unsafe extern "C" fn get_bit(mut fin: *mut byte) -> libc::c_int {
    let mut t: libc::c_int = 0;
    t =
        *fin.offset((bloc >> 3i32) as isize) as libc::c_int >> (bloc & 7i32) &
            0x1i32;
    bloc += 1;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn Huff_Receive(mut node: *mut node_t,
                                      mut ch: *mut libc::c_int,
                                      mut fin: *mut byte) -> libc::c_int {
    while !node.is_null() && (*node).symbol == 256i32 + 1i32 {
        if 0 != get_bit(fin) {
            node = (*node).right
        } else { node = (*node).left }
    }
    if node.is_null() { return 0i32 }
    *ch = (*node).symbol;
    return *ch;
}
#[no_mangle]
pub unsafe extern "C" fn Huff_Init(mut huff: *mut huffman_t) {
    memset(&mut (*huff).compressor as *mut huff_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<huff_t>() as libc::c_ulong);
    memset(&mut (*huff).decompressor as *mut huff_t as *mut libc::c_void,
           0i32, ::std::mem::size_of::<huff_t>() as libc::c_ulong);
    let fresh6 = (*huff).decompressor.blocNode;
    (*huff).decompressor.blocNode = (*huff).decompressor.blocNode + 1;
    (*huff).decompressor.loc[256usize] =
        &mut *(*huff).decompressor.nodeList.as_mut_ptr().offset(fresh6 as
                                                                    isize) as
            *mut node_t;
    (*huff).decompressor.ltail = (*huff).decompressor.loc[256usize];
    (*huff).decompressor.lhead = (*huff).decompressor.ltail;
    (*huff).decompressor.tree = (*huff).decompressor.lhead;
    (*(*huff).decompressor.tree).symbol = 256i32;
    (*(*huff).decompressor.tree).weight = 0i32;
    (*(*huff).decompressor.lhead).prev = 0 as *mut nodetype;
    (*(*huff).decompressor.lhead).next = (*(*huff).decompressor.lhead).prev;
    (*(*huff).decompressor.tree).right = 0 as *mut nodetype;
    (*(*huff).decompressor.tree).left = (*(*huff).decompressor.tree).right;
    (*(*huff).decompressor.tree).parent = (*(*huff).decompressor.tree).left;
    let fresh7 = (*huff).compressor.blocNode;
    (*huff).compressor.blocNode = (*huff).compressor.blocNode + 1;
    (*huff).compressor.loc[256usize] =
        &mut *(*huff).compressor.nodeList.as_mut_ptr().offset(fresh7 as isize)
            as *mut node_t;
    (*huff).compressor.lhead = (*huff).compressor.loc[256usize];
    (*huff).compressor.tree = (*huff).compressor.lhead;
    (*(*huff).compressor.tree).symbol = 256i32;
    (*(*huff).compressor.tree).weight = 0i32;
    (*(*huff).compressor.lhead).prev = 0 as *mut nodetype;
    (*(*huff).compressor.lhead).next = (*(*huff).compressor.lhead).prev;
    (*(*huff).compressor.tree).right = 0 as *mut nodetype;
    (*(*huff).compressor.tree).left = (*(*huff).compressor.tree).right;
    (*(*huff).compressor.tree).parent = (*(*huff).compressor.tree).left;
}
#[no_mangle]
pub unsafe extern "C" fn Huff_offsetReceive(mut node: *mut node_t,
                                            mut ch: *mut libc::c_int,
                                            mut fin: *mut byte,
                                            mut offset: *mut libc::c_int,
                                            mut maxoffset: libc::c_int) {
    bloc = *offset;
    while !node.is_null() && (*node).symbol == 256i32 + 1i32 {
        if bloc >= maxoffset {
            *ch = 0i32;
            *offset = maxoffset + 1i32;
            return
        }
        if 0 != get_bit(fin) {
            node = (*node).right
        } else { node = (*node).left }
    }
    if node.is_null() { *ch = 0i32; return }
    *ch = (*node).symbol;
    *offset = bloc;
}
#[no_mangle]
pub unsafe extern "C" fn Huff_offsetTransmit(mut huff: *mut huff_t,
                                             mut ch: libc::c_int,
                                             mut fout: *mut byte,
                                             mut offset: *mut libc::c_int,
                                             mut maxoffset: libc::c_int) {
    bloc = *offset;
    send((*huff).loc[ch as usize], 0 as *mut node_t, fout, maxoffset);
    *offset = bloc;
}
#[no_mangle]
pub unsafe extern "C" fn Huff_putBit(mut bit: libc::c_int,
                                     mut fout: *mut byte,
                                     mut offset: *mut libc::c_int) {
    bloc = *offset;
    if bloc & 7i32 == 0i32 {
        *fout.offset((bloc >> 3i32) as isize) = 0i32 as byte
    }
    let ref mut fresh8 = *fout.offset((bloc >> 3i32) as isize);
    *fresh8 = (*fresh8 as libc::c_int | bit << (bloc & 7i32)) as byte;
    bloc += 1;
    *offset = bloc;
}
#[no_mangle]
pub unsafe extern "C" fn Huff_getBit(mut fin: *mut byte,
                                     mut offset: *mut libc::c_int)
 -> libc::c_int {
    let mut t: libc::c_int = 0;
    bloc = *offset;
    t =
        *fin.offset((bloc >> 3i32) as isize) as libc::c_int >> (bloc & 7i32) &
            0x1i32;
    bloc += 1;
    *offset = bloc;
    return t;
}
// don't use if you don't know what you're doing.
#[no_mangle]
pub unsafe extern "C" fn Huff_getBloc() -> libc::c_int { return bloc; }
#[no_mangle]
pub unsafe extern "C" fn Huff_setBloc(mut _bloc: libc::c_int) {
    bloc = _bloc;
}