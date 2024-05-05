#!/usr/bin/env python3
# Copyright lowRISC contributors (OpenTitan project).
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

import argparse
import re
import sys

parser = argparse.ArgumentParser(
    description='Enum generator for pkcs11 constants')
parser.add_argument(
    '--binding',
    nargs='+',
    help='Path to cryptoki-sys/src/bindings/x86_64-unknown-linux-gnu.rs')
parser.add_argument('-o', '--output', help='Name of output file')
parser.add_argument('--serde',
                    default=False,
                    action=argparse.BooleanOptionalAction,
                    help='Derive serde Serialize/Deserialize')
parser.add_argument('--strum',
                    default=False,
                    action=argparse.BooleanOptionalAction,
                    help='Derive string conversions with strum')
parser.add_argument('--conv_data',
                    default=False,
                    action=argparse.BooleanOptionalAction,
                    help='Generate AttrData conversions')
parser.add_argument('prefix',
                    metavar='PREFIX',
                    help='A constant prefix like CKO or CKA')
parser.add_argument(
    'rust_ki_type',
    metavar='RUST_KI_TYPE',
    help=
    'Equivalent fully-qualified type in cryptoki (e.g. cryptoki::object::KeyType)'
)

FILE_HEADER = """// Copyright lowRISC contributors (OpenTitan project).
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// This file was autogenerated by `//sw/host/hsmtool/scripts/pkcs11_consts.py`.
// Do not edit.'
"""

# The following names are duplicates in the bindgen'ed constants.
DUPLICATE_NAMES = [
    'CKA_ECDSA_PARAMS',
    'CKA_SUBPRIME_BITS',
    'CKK_ECDSA',
    'CKM_ECDSA_KEY_PAIR_GEN',
]


class EnumGen(object):

    def __init__(self,
                 bindings,
                 prefix,
                 ki_type,
                 serde=False,
                 strum=False,
                 conv_data=False):
        """Create an EnumGen instance.

        Args:
            bindings: List[str]; List of bindings files to process.
            prefix: str; The prefix of the PKCS#11 constant to process.
            ki_type: str; The fully qualified type in cryptoki.
            serde: bool; Whether to derive serde.
            strum: bool; Whether to derive string conversions.
            conv_data: bool; Whether to create AttrData conversion functions.
        """
        self.bindings = bindings
        self.prefix = prefix
        self.ki_type = ki_type
        self.serde = serde
        self.strum = strum
        self.conv_data = conv_data
        self.output = []

    def emit(self, v=''):
        """Emit rust output.

        Args:
            v: str; The text to output.
        """
        self.output.append(v)

    def get_names(self):
        """Get the symbols and type associated with PKCS#1 constant classes.

        For a given class of constants (e.g. CKA, Attributes), get all of
        the symbols with that prefix and their type.

        Returns:
            Tuple[List[str], str]: Symbols and type names.
        """
        names = []
        tp = None
        # Match rust PKCS#11 constants with prefix `self.prefix`.  The constant
        # name goes into match group 1 and the type into match group 2.
        rx = re.compile(r'pub const ({}_.*):\s+(.*)\s+=.*;'.format(
            self.prefix))
        for binding in self.bindings:
            with open(binding, 'rt') as f:
                for line in f:
                    if m := rx.match(line):
                        name = m.group(1)
                        # Eliminate duplicated constants.
                        if name in DUPLICATE_NAMES or 'CAST5' in name:
                            continue
                        if not tp:
                            tp = m.group(2)
                        if m.group(2) != tp:
                            raise Exception(
                                'Expected objects to all be of uniform type',
                                name, tp, m.group(2))
                        names.append(name)
        return (names, tp)

    @staticmethod
    def to_studlycaps(name):
        """Convert a string from snake_case to StudlyCaps.

        Args:
            name; str: The snake_case string.
        Returns:
            The StudlyCaps form of the string.
        """
        words = []
        prev = ''
        for word in name.split('_'):
            if word and word[0].isalpha():
                word = word.capitalize()
                prev = word
            else:
                if prev.isdigit():
                    word = '_' + word
                prev = word
            words.append(word)
        return ''.join(words)

    def emit_enum(self, names):
        """Emit an enum.

        Emits an enum with all of the named enumerators.

        Args:
            names: List[str]; List of PKCS#11 constant names.
        """
        typename = self.ki_type.split('::')[-1]
        prefix = len(self.prefix) + 1
        enumerators = map(lambda name: self.to_studlycaps(name[prefix:]),
                          names)
        traits = [
            'Clone', 'Copy', 'Debug', 'PartialEq', 'Eq', 'Hash',
            'IntoPrimitive', 'FromPrimitive'
        ]
        if self.serde:
            traits.extend(['serde::Serialize', 'serde::Deserialize'])
        if self.strum:
            traits.extend(
                ['strum::Display', 'strum::EnumString', 'strum::EnumIter'])
        self.emit(f'#[derive({", ".join(traits)})]')
        self.emit('#[repr(u64)]')
        self.emit(f'pub enum {typename} {{')
        for (en, val) in zip(enumerators, names):
            snake_case = val[4:].lower()
            if self.serde:
                self.emit(f'    #[serde(rename="{val}")]')
            if self.strum:
                self.emit(
                    f'    #[strum(serialize="{val}", serialize="{en}", serialize="{snake_case}")]'
                )
            self.emit(f'    {en} = {val},')
        self.emit('    #[num_enum(catch_all)]')
        self.emit(f'    Unknown{typename}(u64) = u64::MAX,')
        self.emit('}')

    def emit_conversions(self, reprtype):
        """Emit conversion functions.

        Args:
            reprtype: str; The low-level PKCS#11 representation type.
        """
        typename = self.ki_type.split('::')[-1]
        self.emit(f"""
impl From<{self.ki_type}> for {typename} {{
    fn from(val: {self.ki_type}) -> Self {{
        let val = {reprtype}::from(val);
        Self::from(val)
    }}
}}

impl TryFrom<{typename}> for {self.ki_type} {{
    type Error = cryptoki::error::Error;
    fn try_from(val: {typename}) -> Result<Self, Self::Error> {{
        let val = {reprtype}::from(val);
        {self.ki_type}::try_from(val)
    }}
}}
""")

        if self.conv_data:
            self.emit(f"""
impl TryFrom<&AttrData> for {typename} {{
    type Error = AttributeError;
    fn try_from(val: &AttrData) -> Result<Self, Self::Error> {{
        match val {{
            AttrData::{typename}(x) => Ok(*x),
            _ => Err(AttributeError::EncodingError),
        }}
    }}
}}

impl From<{typename}> for AttrData {{
    fn from(val: {typename}) -> Self {{
        AttrData::{typename}(val)
    }}
}}
""")

    def emit_file(self):
        """Emit rust source code for the enum and conversion traits."""
        (names, reprtype) = self.get_names()
        self.emit(FILE_HEADER)

        self.emit('use std::convert::TryFrom;')
        self.emit('use cryptoki_sys::*;')
        self.emit('use num_enum::{IntoPrimitive, FromPrimitive};')
        self.emit()
        if self.conv_data:
            self.emit(
                'use crate::util::attribute::{AttrData, AttributeError};')
        self.emit()
        self.emit_enum(names)
        self.emit_conversions(reprtype)
        return '\n'.join(self.output)


def main(argv):
    args = parser.parse_args()
    gen = EnumGen(args.binding, args.prefix, args.rust_ki_type, args.serde,
                  args.strum, args.conv_data)
    out = gen.emit_file()
    print(out)
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv))
