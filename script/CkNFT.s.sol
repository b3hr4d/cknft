// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "../lib/forge-std/src/Script.sol";
import "../backend/ethereum/CkNFT.sol";

contract CkNFTScript is Script {
  function setUp() public {}

  function run() public {
    vm.broadcast();

    new CkNFT{salt: bytes32(uint256(2))}(
      0xB51f94aEEebE55A3760E8169A22e536eBD3a6DCB
    );
  }
}
