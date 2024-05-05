// Copyright lowRISC contributors (OpenTitan project).
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
//
// DO NOT EDIT THIS FILE DIRECTLY.
//
// It has been generated with:
// $ ./util/design/gen-lc-state-enc.py
//     --seed ${lc_st_enc.config['seed']}
//     --raw-unlock-rs-template=sw/device/silicon_creator/manuf/data/lc_raw_unlock_token.rs.tpl

#![allow(dead_code)]

//! LC unlock tokens embedded in the HW as netlist constants.

<%!
    import re

    # Regex to find the spaces between words in a PascalCase name
    regex = re.compile(r'(?<!^)(?=[A-Z])')

    # Convert PascalCase names to SCREAMING_SNAKE_CASE
    def pascal_to_screaming_snake(pascal):
        return regex.sub('_', pascal).upper()
%>

% for token in lc_st_enc.config['tokens']:
<%
    token_name = pascal_to_screaming_snake(token['name'])
    token_value = "{0:0X}".format(token['value'])
%>
pub const ${token_name}: u128 = 0x${token_value};
% endfor
