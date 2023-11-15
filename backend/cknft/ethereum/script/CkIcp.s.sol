// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "../src/CkIcp.sol";

contract CkIcpScript is Script {
  function setUp() public {}

  function run() public {
    vm.broadcast();
    new CkIcp{salt: bytes32(uint256(1))}(
      address(0xB51f94aEEebE55A3760E8169A22e536eBD3a6DCB)
    );
  }
}
