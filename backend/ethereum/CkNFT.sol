// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC1155/extensions/ERC1155Pausable.sol";
import "@openzeppelin/contracts/token/ERC1155/extensions/ERC1155Burnable.sol";
import "@openzeppelin/contracts/token/ERC1155/extensions/ERC1155Supply.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

contract CkNFT is
  ERC1155,
  Ownable,
  ERC1155Pausable,
  ERC1155Burnable,
  ERC1155Supply
{
  using ECDSA for bytes32;

  event SelfMint(uint256 indexed msgid);
  event BurnToCkNFT(
    uint256 id,
    bytes32 indexed principal,
    bytes32 indexed subaccount
  );

  mapping(uint256 => bool) public used;

  constructor(
    address initialOwner
  )
    ERC1155("https://i6s5o-xaaaa-aaaap-abrmq-cai.raw.icp0.io/evm/")
    Ownable(initialOwner)
  {}

  function setURI(string memory newuri) public onlyOwner {
    _setURI(newuri);
  }

  function pause() public onlyOwner {
    _pause();
  }

  function unpause() public onlyOwner {
    _unpause();
  }

  function mint(
    address account,
    uint256 id,
    uint256 amount,
    bytes memory data
  ) public onlyOwner {
    _mint(account, id, amount, data);
  }

  function mintBatch(
    address to,
    uint256[] memory ids,
    uint256[] memory amounts,
    bytes memory data
  ) public onlyOwner {
    _mintBatch(to, ids, amounts, data);
  }

  function baseURI() public view returns (string memory) {
    return super.uri(0);
  }

  /**
   * @dev Returns an URI for a given token ID
   */
  function tokenURI(uint256 _tokenId) public view returns (string memory) {
    uint chainId = block.chainid;

    return
      string(
        abi.encodePacked(
          baseURI(),
          "cknft/",
          Strings.toString(chainId),
          "/",
          Strings.toString(_tokenId)
        )
      );
  }

  function selfMint(
    uint256 id,
    address to,
    uint256 msgid,
    uint64 expiry,
    bytes calldata signature
  ) public whenNotPaused {
    require(block.timestamp < expiry, "Signature expired");
    require(!used[msgid], "MsgId already used");
    require(
      _verifyOwnerSignature(
        keccak256(
          abi.encode(id, to, msgid, expiry, block.chainid, address(this))
        ),
        signature
      ),
      "Invalid signature"
    );
    used[msgid] = true;
    _mint(to, id, 1, "");

    emit SelfMint(msgid);
  }

  function burnToCkNFT(uint256 id, bytes32 accountId) public whenNotPaused {
    _burn(_msgSender(), id, 1);

    emit BurnToCkNFT(id, accountId, 0);
  }

  function burnToCkNFT(
    uint256 id,
    bytes32 principal,
    bytes32 subaccount
  ) public whenNotPaused {
    _burn(_msgSender(), id, 1);

    emit BurnToCkNFT(id, principal, subaccount);
  }

  // The following functions are overrides required by Solidity.

  function _verifyOwnerSignature(
    bytes32 hash,
    bytes calldata signature
  ) internal view returns (bool) {
    return hash.recover(signature) == owner();
  }

  function _update(
    address from,
    address to,
    uint256[] memory ids,
    uint256[] memory values
  ) internal override(ERC1155, ERC1155Pausable, ERC1155Supply) {
    super._update(from, to, ids, values);
  }
}
