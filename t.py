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

def conversion_bc(x: str, y: str) -> str:
    return f"""impl From<{x}> for {y} {{
    fn from(x: {x}) -> Self {{
        Self {{
            black: x.black.into(),
            white: x.white.into(),
        }}
    }}
}}"""

def conversion_back_and_forth_bc(x: str, y: str) -> str:
    return conversion_bc(x,y) + conversion_bc(y,x)

def decl_by_color(t: str) -> str:
   return fr"""#[derive(Copy, Clone, Default, Eq, PartialEq, Debug, Hash)]
    #[repr(C)]
    pub struct ByColor{t}Ffi {{
        pub black: {t}Ffi,
        pub white: {t}Ffi,
    }}"""

def monomorphise_bc(l: List[str]):

    for ty in l:
        original = f"ByColor<{ty}>"
        ffi = f"ByColor{ty}Ffi"
        insert_code("ffi", 4, decl_by_color(ty))
        insert_code("color", 414, conversion_back_and_forth_bc(original, ffi))
        def replace(src: List[str]) -> str:
            file = "".join(src)
            return file.replace(original, ffi)
        change_file_in_memory("ffi", replace)

def insert_code(file_name: str, line_nb: int, string: str):
    def inner(src: str) -> str:
        src.insert(line_nb - 1, f"{string}\n")
        return "".join(src)
    change_file_in_memory(file_name, inner)


def main() -> None:
    insert_code("lib", 97, "mod ffi;")
    insert_code("bitboard", 1235, BB_DEBUG)
    insert_code("square", 968, SQUARE_DEBUG)
    monomorphise_bc(["Bitboard"])

########
# Main #
########

if __name__ == "__main__":
    print('#'*80)
    main()