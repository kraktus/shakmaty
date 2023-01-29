#!/usr/local/bin/python3
#coding: utf-8
# Licence: GNU AGPLv3

""""""

from __future__ import annotations

import argparse
import json
import logging
import logging.handlers
import os
import sys

from dataclasses import dataclass
from datetime import datetime
from collections import deque
from pathlib import Path
from typing import Optional, List, Union, Tuple



###########
# Helpers #
###########


def decl_by_color(name: str, ty: str) -> str:
   return f"""
    #[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
    pub struct ByColor{name}Ffi {{
        pub black: {ty},
        pub white: {ty},
    }}
    """

def decl_by_role(name: str, ty: str) -> str:
    return f"""
    #[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
    pub struct ByRole{name}Ffi {{
        pub pawn: {ty},
        pub knight: {ty},
        pub bishop: {ty},
        pub rook: {ty},
        pub queen: {ty},
        pub king: {ty},
    }}
    """

def conversion_bc(x: str, y: str) -> str:
    return f"""impl From<ByColor{x}> for ByColor{y} {{
    fn from(x: ByColor{x}) -> Self {{
        Self {{
            black: x.black.into(),
            white: x.white.into(),
        }}
    }}
}}
"""

def conversion_br(x: str, y: str) -> str:
    return f"""impl From<ByRole{x}> for ByRole{y} {{
    fn from(x: ByRole{x}) -> Self {{
        Self {{
            pawn: x.pawn.into(),
            knight: x.knight.into(),
            bishop: x.bishop.into(),
            rook: x.rook.into(),
            queen: x.queen.into(),
            king: x.king.into(),
        }}
    }}
}}
"""

def conversion_back_and_forth_bc(x: str, y: str) -> str:
    return conversion_bc(x,y) + conversion_bc(y,x)

def conversion_back_and_forth_br(x: str, y: str) -> str:
    return conversion_br(x,y) + conversion_br(y,x)

#############
# Constants #
#############

BB_DEBUG = """impl fmt::Debug for BitboardFfi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = Bitboard::from(self.clone());
        res.fmt(f)
    }
}
"""
SQUARE_DEBUG = """
impl fmt::Debug for SquareFfi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = Square::from(self.clone());
        res.fmt(f)
    }
}
"""

BY_COLOR_CONVERSIONS = f"""
{conversion_back_and_forth_bc("<crate::Bitboard>", "BitboardFfi")}
{conversion_back_and_forth_bc("<ByRole<u8>>", "ByRoleU8Ffi")}
{conversion_back_and_forth_bc("<RemainingChecks>", "RemainingChecksFfi")}
{conversion_back_and_forth_bc("<[OptionSquareFfi; 2]>", "Array2OptionSquare")}
{conversion_back_and_forth_bc("<[BitboardFfi; 2]>", "Array2Bitboard")}
"""

BY_COLOR_DEF = f"""
    {decl_by_color("Bitboard", "BitboardFfi")}
    {decl_by_color("ByRoleU8", "ByRoleU8Ffi")}
    {decl_by_color("RemainingChecks", "RemainingChecksFfi")}
    {decl_by_color("Array2OptionSquare", "[OptionSquareFfi; 2]")}
    {decl_by_color("Array2Bitboard", "[BitboardFfi; 2]")}
"""

BY_ROLE_CONVERSIONS = f"""
{conversion_back_and_forth_br("<u8>", "U8Ffi")}
{conversion_back_and_forth_br("<crate::Bitboard>", "BitboardFfi")}
"""

BY_ROLE_DEF = f"""
    {decl_by_role("U8", "u8")}
    {decl_by_role("Bitboard", "BitboardFfi")}
"""

########
# Logs #
########

###########
# Classes #
###########


def change_file_in_memory(file_name: str, fn):
    with open(f"src/{file_name}.rs", "r") as f:
        src = f.readlines()
    with open(f"src/{file_name}.rs", "w") as f:
        f.write(fn(src))

def monomorphise_bc(l: List[str]):
    for ty in l:
        while t[-3:] == "Ffi":
            t = t[:-3]
        original = f"ByColor<crate::{ty}>"
        ffi = f"ByColor{ty}Ffi"
        insert_code("ffi", 4, decl_by_color(ty))
        insert_code("color", 414, conversion_back_and_forth_bc(original, ffi))
        def replace(src: List[str]) -> str:
            file = "".join(src)
            return file.replace(f"ByColor<{ty}Ffi>", ffi)
        change_file_in_memory("ffi", replace)

def monomorphise_br(l: List[str]):
    for ty in l:
        original = f"ByRole<crate::{ty}>"
        ffi = f"ByRole{ty}Ffi"
        insert_code("ffi", 4, decl_by_role(ty, False))
        insert_code("role", 368, conversion_back_and_forth_br(original, ffi))
        def replace(src: List[str]) -> str:
            file = "".join(src)
            return file.replace(f"ByRole<{ty}Ffi>", ffi)
        change_file_in_memory("ffi", replace)

def monomorphise_br_primitive(l: List[str]):
    for ty in l:
        original = f"ByRole<{ty}>"
        ffi = f"ByRole{ty}Ffi"
        insert_code("ffi", 4, decl_by_role(ty, True))
        insert_code("role", 368, conversion_back_and_forth_br(original, ffi))
        def replace(src: List[str]) -> str:
            file = "".join(src)
            return file.replace(f"ByRole<{ty}>", ffi)
        change_file_in_memory("ffi", replace)

def insert_code(file_name: str, line_nb: int, string: str):
    def inner(src: str) -> str:
        src.insert(line_nb - 1, f"{string}\n")
        return "".join(src)
    change_file_in_memory(file_name, inner)

def replace_ffi(x: str, by: str):
    def replace(src: List[str]) -> str:
            file = "".join(src)
            return file.replace(x, by)
    change_file_in_memory("ffi", replace)


def main() -> None:
    insert_code("lib", 97, "mod ffi;")
    insert_code("bitboard", 1263, BB_DEBUG)
    insert_code("square", 968, SQUARE_DEBUG)
    insert_code("ffi", 4, BY_COLOR_DEF)
    insert_code("ffi", 4, BY_ROLE_DEF)
    insert_code("role", 368, BY_ROLE_CONVERSIONS)
    insert_code("color", 414, BY_COLOR_CONVERSIONS)
    # By Color
    replace_ffi("ByColor<BitboardFfi>", "ByColorBitboardFfi")
    replace_ffi("ByColor<ByRole<u8>>", "ByColorByRoleU8Ffi")
    replace_ffi("ByColor<RemainingChecksFfi>", "ByColorRemainingChecksFfi")
    replace_ffi("ByColor<[OptionSquareFfi; 2]>", "ByColorArray2OptionSquareFfi")
    replace_ffi("ByColor<[BitboardFfi; 2]>", "ByColorArray2BitboardFfi")
    # By Role
    replace_ffi("ByRole<u8>", "ByRoleU8Ffi")
    replace_ffi("ByRole<BitboardFfi>", "ByRoleBitboardFfi")


########
# Main #
########

if __name__ == "__main__":
    print('#'*80)
    main()