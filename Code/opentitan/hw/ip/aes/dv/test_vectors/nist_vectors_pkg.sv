// Copyright lowRISC contributors (OpenTitan project).
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

//##############################################
//#  https://csrc.nist.gov/CSRC/media/Projects/Cryptographic
//#-Standards-and-Guidelines/documents/examples/AES_ModesA_All.pdf
//###############################################
//#             AES TEST VECTORS                #
//#  Modes: ECB, CTR, OFB, CFB  CBC             #
//#  Key Lengths: 128bit, 192bit, 256bit        #
//#                                             #
//#  This File was autogenerated by an external #
//#  script converting the PDF to systemverilog #
//###############################################
package nist_vectors_pkg;

  import uvm_pkg::*;
  import aes_pkg::*;

  typedef struct {
    aes_mode_e   mode;
    key_len_e    key_len;
    aes_op_e     operation;
    bit [127:0]  iv;
    bit [255:0]  key;
    bit [3:0] [31:0]  plain_text[4];
    bit [3:0] [31:0]  cipher_text[4];
  } nist_vector_t;



  function automatic string vector2string(nist_vector_t vector);
    string              str;
    str = $sformatf("\n ----| NIST Vector | ----");
    str = $sformatf("%s \n Notes that the Nist Vectors are listed in little endian format",
                    str);
    str = $sformatf("%s \n and need to be byteswapped for use with AES", str);
    str = $sformatf("%s \n Mode: %s", str, vector.mode.name);
    str = $sformatf("%s \n Key Len: %s", str, vector.key_len.name);
    str = $sformatf("%s \n Key: %0h", str, vector.key);
    str = $sformatf("%s \n Iv: %0h", str, vector.iv);
    str = $sformatf("%s \n plaintext: %0h", str, vector.plain_text[0]);
    str = $sformatf("%s \n plaintext: %0h", str, vector.plain_text[1]);
    str = $sformatf("%s \n plaintext: %0h", str, vector.plain_text[2]);
    str = $sformatf("%s \n plaintext: %0h", str, vector.plain_text[3]);
    str = $sformatf("%s \n ciphertext: %0h", str, vector.cipher_text[0]);
    str = $sformatf("%s \n ciphertext: %0h", str, vector.cipher_text[1]);
    str = $sformatf("%s \n ciphertext: %0h", str, vector.cipher_text[2]);
    str = $sformatf("%s \n ciphertext: %0h", str, vector.cipher_text[3]);
    return str;
  endfunction // vector2string


  class aes_nist_vectors extends uvm_object;
    `uvm_object_utils(aes_nist_vectors)
    nist_vector_t vector_q[];
    int           num;


    function new(string name = "aes_nist_vectors");
      super.new();
      num = get_num_vectors();
      vector_q = new[num];
      get_vectors(vector_q);
    endfunction // new


    function void get_vectors(ref nist_vector_t nist_vectors[]);


      // NIST VECTOR[0] //
      nist_vectors[0].mode    = AES_ECB;
      nist_vectors[0].key_len = AES_128;
      nist_vectors[0].key  = 256'h2B7E151628AED2A6ABF7158809CF4F3C00000000000000000000000000000000;
      nist_vectors[0].iv      = 128'h00000000000000000000000000000000;
      nist_vectors[0].plain_text  = '{ 128'h6BC1BEE22E409F96E93D7E117393172A,
                                       128'hAE2D8A571E03AC9C9EB76FAC45AF8E51,
                                       128'h30C81C46A35CE411E5FBC1191A0A52EF,
                                       128'hF69F2445DF4F9B17AD2B417BE66C3710
                                      };
      nist_vectors[0].cipher_text = '{ 128'h3AD77BB40D7A3660A89ECAF32466EF97,
                                       128'hF5D3D58503B9699DE785895A96FDBAAF,
                                       128'h43B1CD7F598ECE23881B00E3ED030688,
                                       128'h7B0C785E27E8AD3F8223207104725DD4
                                      };



      // NIST VECTOR[1] //
      nist_vectors[1].mode    = AES_ECB;
      nist_vectors[1].key_len = AES_192;
      nist_vectors[1].key  = 256'h8E73B0F7DA0E6452C810F32B809079E562F8EAD2522C6B7B0000000000000000;
      nist_vectors[1].iv      = 128'h00000000000000000000000000000000;
      nist_vectors[1].plain_text  = '{ 128'h6BC1BEE22E409F96E93D7E117393172A,
                                       128'hAE2D8A571E03AC9C9EB76FAC45AF8E51,
                                       128'h30C81C46A35CE411E5FBC1191A0A52EF,
                                       128'hF69F2445DF4F9B17AD2B417BE66C3710
                                      };
      nist_vectors[1].cipher_text = '{ 128'hBD334F1D6E45F25FF712A214571FA5CC,
                                       128'h974104846D0AD3AD7734ECB3ECEE4EEF,
                                       128'hEF7AFD2270E2E60ADCE0BA2FACE6444E,
                                       128'h9A4B41BA738D6C72FB16691603C18E0E
                                      };



      // NIST VECTOR[2] //
      nist_vectors[2].mode    = AES_ECB;
      nist_vectors[2].key_len = AES_256;
      nist_vectors[2].key  = 256'h603DEB1015CA71BE2B73AEF0857D77811F352C073B6108D72D9810A30914DFF4;
      nist_vectors[2].iv      = 128'h00000000000000000000000000000000;
      nist_vectors[2].plain_text  = '{ 128'h6BC1BEE22E409F96E93D7E117393172A,
                                       128'hAE2D8A571E03AC9C9EB76FAC45AF8E51,
                                       128'h30C81C46A35CE411E5FBC1191A0A52EF,
                                       128'hF69F2445DF4F9B17AD2B417BE66C3710
                                      };
      nist_vectors[2].cipher_text = '{ 128'hF3EED1BDB5D2A03C064B5A7E3DB181F8,
                                       128'h591CCB10D410ED26DC5BA74A31362870,
                                       128'hB6ED21B99CA6F4F9F153E7B1BEAFED1D,
                                       128'h23304B7A39F9F3FF067D8D8F9E24ECC7
                                      };



      // NIST VECTOR[3] //
      nist_vectors[3].mode    = AES_CBC;
      nist_vectors[3].key_len = AES_128;
      nist_vectors[3].key  = 256'h2B7E151628AED2A6ABF7158809CF4F3C00000000000000000000000000000000;
      nist_vectors[3].iv      = 128'h000102030405060708090A0B0C0D0E0F;
      nist_vectors[3].plain_text  = '{ 128'h6BC1BEE22E409F96E93D7E117393172A,
                                       128'hAE2D8A571E03AC9C9EB76FAC45AF8E51,
                                       128'h30C81C46A35CE411E5FBC1191A0A52EF,
                                       128'hF69F2445DF4F9B17AD2B417BE66C3710
                                      };
      nist_vectors[3].cipher_text = '{ 128'h7649ABAC8119B246CEE98E9B12E9197D,
                                       128'h5086CB9B507219EE95DB113A917678B2,
                                       128'h73BED6B8E3C1743B7116E69E22229516,
                                       128'h3FF1CAA1681FAC09120ECA307586E1A7
                                      };



      // NIST VECTOR[4] //
      nist_vectors[4].mode    = AES_CBC;
      nist_vectors[4].key_len = AES_192;
      nist_vectors[4].key  = 256'h8E73B0F7DA0E6452C810F32B809079E562F8EAD2522C6B7B0000000000000000;
      nist_vectors[4].iv      = 128'h000102030405060708090A0B0C0D0E0F;
      nist_vectors[4].plain_text  = '{ 128'h6BC1BEE22E409F96E93D7E117393172A,
                                       128'hAE2D8A571E03AC9C9EB76FAC45AF8E51,
                                       128'h30C81C46A35CE411E5FBC1191A0A52EF,
                                       128'hF69F2445DF4F9B17AD2B417BE66C3710
                                      };
      nist_vectors[4].cipher_text = '{ 128'h4F021DB243BC633D7178183A9FA071E8,
                                       128'hB4D9ADA9AD7DEDF4E5E738763F69145A,
                                       128'h571B242012FB7AE07FA9BAAC3DF102E0,
                                       128'h08B0E27988598881D920A9E64F5615CD
                                      };



      // NIST VECTOR[5] //
      nist_vectors[5].mode    = AES_CBC;
      nist_vectors[5].key_len = AES_256;
      nist_vectors[5].key  = 256'h603DEB1015CA71BE2B73AEF0857D77811F352C073B6108D72D9810A30914DFF4;
      nist_vectors[5].iv      = 128'h000102030405060708090A0B0C0D0E0F;
      nist_vectors[5].plain_text  = '{ 128'h6BC1BEE22E409F96E93D7E117393172A,
                                       128'hAE2D8A571E03AC9C9EB76FAC45AF8E51,
                                       128'h30C81C46A35CE411E5FBC1191A0A52EF,
                                       128'hF69F2445DF4F9B17AD2B417BE66C3710
                                      };
      nist_vectors[5].cipher_text = '{ 128'hF58C4C04D6E5F1BA779EABFB5F7BFBD6,
                                       128'h9CFC4E967EDB808D679F777BC6702C7D,
                                       128'h39F23369A9D9BACFA530E26304231461,
                                       128'hB2EB05E2C39BE9FCDA6C19078C6A9D1B
                                      };



      // NIST VECTOR[6] //
      nist_vectors[6].mode    = AES_CFB;
      nist_vectors[6].key_len = AES_128;
      nist_vectors[6].key  = 256'h2B7E151628AED2A6ABF7158809CF4F3C00000000000000000000000000000000;
      nist_vectors[6].iv      = 128'h000102030405060708090A0B0C0D0E0F;
      nist_vectors[6].plain_text  = '{ 128'h6BC1BEE22E409F96E93D7E117393172A,
                                       128'hAE2D8A571E03AC9C9EB76FAC45AF8E51,
                                       128'h30C81C46A35CE411E5FBC1191A0A52EF,
                                       128'hF69F2445DF4F9B17AD2B417BE66C3710
                                      };
      nist_vectors[6].cipher_text = '{ 128'h3B3FD92EB72DAD20333449F8E83CFB4A,
                                       128'hC8A64537A0B3A93FCDE3CDAD9F1CE58B,
                                       128'h26751F67A3CBB140B1808CF187A4F4DF,
                                       128'hC04B05357C5D1C0EEAC4C66F9FF7F2E6
                                      };



      // NIST VECTOR[7] //
      nist_vectors[7].mode    = AES_CFB;
      nist_vectors[7].key_len = AES_192;
      nist_vectors[7].key  = 256'h8E73B0F7DA0E6452C810F32B809079E562F8EAD2522C6B7B0000000000000000;
      nist_vectors[7].iv      = 128'h000102030405060708090A0B0C0D0E0F;
      nist_vectors[7].plain_text  = '{ 128'h6BC1BEE22E409F96E93D7E117393172A,
                                       128'hAE2D8A571E03AC9C9EB76FAC45AF8E51,
                                       128'h30C81C46A35CE411E5FBC1191A0A52EF,
                                       128'hF69F2445DF4F9B17AD2B417BE66C3710
                                      };
      nist_vectors[7].cipher_text = '{ 128'hCDC80D6FDDF18CAB34C25909C99A4174,
                                       128'h67CE7F7F81173621961A2B70171D3D7A,
                                       128'h2E1E8A1DD59B88B1C8E60FED1EFAC4C9,
                                       128'hC05F9F9CA9834FA042AE8FBA584B09FF
                                      };



      // NIST VECTOR[8] //
      nist_vectors[8].mode    = AES_CFB;
      nist_vectors[8].key_len = AES_256;
      nist_vectors[8].key  = 256'h603DEB1015CA71BE2B73AEF0857D77811F352C073B6108D72D9810A30914DFF4;
      nist_vectors[8].iv      = 128'h000102030405060708090A0B0C0D0E0F;
      nist_vectors[8].plain_text  = '{ 128'h6BC1BEE22E409F96E93D7E117393172A,
                                       128'hAE2D8A571E03AC9C9EB76FAC45AF8E51,
                                       128'h30C81C46A35CE411E5FBC1191A0A52EF,
                                       128'hF69F2445DF4F9B17AD2B417BE66C3710
                                      };
      nist_vectors[8].cipher_text = '{ 128'hDC7E84BFDA79164B7ECD8486985D3860,
                                       128'h39FFED143B28B1C832113C6331E5407B,
                                       128'hDF10132415E54B92A13ED0A8267AE2F9,
                                       128'h75A385741AB9CEF82031623D55B1E471
                                      };



      // NIST VECTOR[9] //
      nist_vectors[9].mode    = AES_OFB;
      nist_vectors[9].key_len = AES_128;
      nist_vectors[9].key  = 256'h2B7E151628AED2A6ABF7158809CF4F3C00000000000000000000000000000000;
      nist_vectors[9].iv      = 128'h000102030405060708090A0B0C0D0E0F;
      nist_vectors[9].plain_text  = '{ 128'h6BC1BEE22E409F96E93D7E117393172A,
                                       128'hAE2D8A571E03AC9C9EB76FAC45AF8E51,
                                       128'h30C81C46A35CE411E5FBC1191A0A52EF,
                                       128'hF69F2445DF4F9B17AD2B417BE66C3710
                                      };
      nist_vectors[9].cipher_text = '{ 128'h3B3FD92EB72DAD20333449F8E83CFB4A,
                                       128'h7789508D16918F03F53C52DAC54ED825,
                                       128'h9740051E9C5FECF64344F7A82260EDCC,
                                       128'h304C6528F659C77866A510D9C1D6AE5E
                                      };



      // NIST VECTOR[10] //
      nist_vectors[10].mode    = AES_OFB;
      nist_vectors[10].key_len = AES_192;
      nist_vectors[10].key  = 256'h8E73B0F7DA0E6452C810F32B809079E562F8EAD2522C6B7B0000000000000000;
      nist_vectors[10].iv      = 128'h000102030405060708090A0B0C0D0E0F;
      nist_vectors[10].plain_text  = '{ 128'h6BC1BEE22E409F96E93D7E117393172A,
                                       128'hAE2D8A571E03AC9C9EB76FAC45AF8E51,
                                       128'h30C81C46A35CE411E5FBC1191A0A52EF,
                                       128'hF69F2445DF4F9B17AD2B417BE66C3710
                                      };
      nist_vectors[10].cipher_text = '{ 128'hCDC80D6FDDF18CAB34C25909C99A4174,
                                       128'hFCC28B8D4C63837C09E81700C1100401,
                                       128'h8D9A9AEAC0F6596F559C6D4DAF59A5F2,
                                       128'h6D9F200857CA6C3E9CAC524BD9ACC92A
                                      };



      // NIST VECTOR[11] //
      nist_vectors[11].mode    = AES_OFB;
      nist_vectors[11].key_len = AES_256;
      nist_vectors[11].key  = 256'h603DEB1015CA71BE2B73AEF0857D77811F352C073B6108D72D9810A30914DFF4;
      nist_vectors[11].iv      = 128'h000102030405060708090A0B0C0D0E0F;
      nist_vectors[11].plain_text  = '{ 128'h6BC1BEE22E409F96E93D7E117393172A,
                                       128'hAE2D8A571E03AC9C9EB76FAC45AF8E51,
                                       128'h30C81C46A35CE411E5FBC1191A0A52EF,
                                       128'hF69F2445DF4F9B17AD2B417BE66C3710
                                      };
      nist_vectors[11].cipher_text = '{ 128'hDC7E84BFDA79164B7ECD8486985D3860,
                                       128'h4FEBDC6740D20B3AC88F6AD82A4FB08D,
                                       128'h71AB47A086E86EEDF39D1C5BBA97C408,
                                       128'h0126141D67F37BE8538F5A8BE740E484
                                      };



      // NIST VECTOR[12] //
      nist_vectors[12].mode    = AES_CTR;
      nist_vectors[12].key_len = AES_128;
      nist_vectors[12].key  = 256'h2B7E151628AED2A6ABF7158809CF4F3C00000000000000000000000000000000;
      nist_vectors[12].iv      = 128'hF0F1F2F3F4F5F6F7F8F9FAFBFCFDFEFF;
      nist_vectors[12].plain_text  = '{ 128'h6BC1BEE22E409F96E93D7E117393172A,
                                       128'hAE2D8A571E03AC9C9EB76FAC45AF8E51,
                                       128'h30C81C46A35CE411E5FBC1191A0A52EF,
                                       128'hF69F2445DF4F9B17AD2B417BE66C3710
                                      };
      nist_vectors[12].cipher_text = '{ 128'h874D6191B620E3261BEF6864990DB6CE,
                                       128'h9806F66B7970FDFF8617187BB9FFFDFF,
                                       128'h5AE4DF3EDBD5D35E5B4F09020DB03EAB,
                                       128'h1E031DDA2FBE03D1792170A0F3009CEE
                                      };



      // NIST VECTOR[13] //
      nist_vectors[13].mode    = AES_CTR;
      nist_vectors[13].key_len = AES_192;
      nist_vectors[13].key  = 256'h8E73B0F7DA0E6452C810F32B809079E562F8EAD2522C6B7B0000000000000000;
      nist_vectors[13].iv      = 128'hF0F1F2F3F4F5F6F7F8F9FAFBFCFDFEFF;
      nist_vectors[13].plain_text  = '{ 128'h6BC1BEE22E409F96E93D7E117393172A,
                                       128'hAE2D8A571E03AC9C9EB76FAC45AF8E51,
                                       128'h30C81C46A35CE411E5FBC1191A0A52EF,
                                       128'hF69F2445DF4F9B17AD2B417BE66C3710
                                      };
      nist_vectors[13].cipher_text = '{ 128'h1ABC932417521CA24F2B0459FE7E6E0B,
                                       128'h090339EC0AA6FAEFD5CCC2C6F4CE8E94,
                                       128'h1E36B26BD1EBC670D1BD1D665620ABF7,
                                       128'h4F78A7F6D29809585A97DAEC58C6B050
                                      };



      // NIST VECTOR[14] //
      nist_vectors[14].mode    = AES_CTR;
      nist_vectors[14].key_len = AES_256;
      nist_vectors[14].key  = 256'h603DEB1015CA71BE2B73AEF0857D77811F352C073B6108D72D9810A30914DFF4;
      nist_vectors[14].iv      = 128'hF0F1F2F3F4F5F6F7F8F9FAFBFCFDFEFF;
      nist_vectors[14].plain_text  = '{ 128'h6BC1BEE22E409F96E93D7E117393172A,
                                       128'hAE2D8A571E03AC9C9EB76FAC45AF8E51,
                                       128'h30C81C46A35CE411E5FBC1191A0A52EF,
                                       128'hF69F2445DF4F9B17AD2B417BE66C3710
                                      };
      nist_vectors[14].cipher_text = '{ 128'h601EC313775789A5B7A7F504BBF3D228,
                                       128'hF443E3CA4D62B59ACA84E990CACAF5C5,
                                       128'h2B0930DAA23DE94CE87017BA2D84988D,
                                       128'hDFC9C58DB67AADA613C2DD08457941A6
                                      };

    endfunction

    function int get_num_vectors();
      return 15;
    endfunction

  endclass
endpackage
