// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "../lib/forge-std/src/Test.sol";
import "../backend/ethereum/CkNFT.sol";

contract CkNFTTest is Test {
  CkNFT ckNFT;

  function setUp() public {
    ckNFT = new CkNFT(0xB51f94aEEebE55A3760E8169A22e536eBD3a6DCB);
  }
}
