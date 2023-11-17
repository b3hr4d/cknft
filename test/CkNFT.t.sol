// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "../lib/forge-std/src/Test.sol";
import "../backend/ethereum/CkNFT.sol";

contract CkNFTTest is Test {
  CkNFT ckNFT;

  uint256 ownerKey =
    0xf3110f00c80c7a75101122807b1dd2c96e37b88570e65815101d72012a2b7671;
  address owner = vm.addr(ownerKey);
  address globalUser;

  constructor() {
    vm.chainId(1);

    ckNFT = new CkNFT(owner);
  }

  function testURI() public {
    vm.prank(owner);

    ckNFT.setURI("https://example.com/");
    assertEq(ckNFT.tokenURI(0), "https://example.com/0");
  }
}
