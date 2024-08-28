// SPDX-License-Identifier: Apache-2.0 AND MIT

pragma solidity 0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721.sol";

/**
 * @title A helper smart contract for ETH/ERC20/NFT <-> ckETH/ckERC20/ckNFT conversion.
 * @notice This smart contract deposits incoming ETH/ERC20/NFT to the ckETH minter account and emits deposit events.
 */
contract CkTokenDeposit {
  using SafeERC20 for IERC20;

  address payable private immutable cketh_minter_main_address;

  event ReceivedEth(
    address indexed from,
    uint256 value,
    bytes32 indexed principal
  );

  event ReceivedErc20(
    address indexed erc20_contract_address,
    address indexed owner,
    uint256 amount,
    bytes32 indexed principal
  );

  event ReceivedNft(
    address indexed nft_contract_address,
    address indexed owner,
    uint256 tokenId,
    bytes32 indexed principal
  );

  /**
   * @dev Set cketh_minter_main_address.
   */
  constructor(address _cketh_minter_main_address) {
    cketh_minter_main_address = payable(_cketh_minter_main_address);
  }

  /**
   * @dev Return ckETH minter main address.
   * @return address of ckETH minter main address.
   */
  function getMinterAddress() public view returns (address) {
    return cketh_minter_main_address;
  }

  /**
   * @dev Emits the `ReceivedEth` event if the transfer succeeds.
   */
  function depositEth(bytes32 _principal) public payable {
    emit ReceivedEth(msg.sender, msg.value, _principal);
    cketh_minter_main_address.transfer(msg.value);
  }

  /**
   * @dev Emits the `ReceivedErc20` event if the transfer succeeds.
   */
  function depositErc20(
    address erc20_address,
    uint256 amount,
    bytes32 principal
  ) public {
    IERC20 erc20Token = IERC20(erc20_address);
    erc20Token.safeTransferFrom(msg.sender, cketh_minter_main_address, amount);

    emit ReceivedErc20(erc20_address, msg.sender, amount, principal);
  }

  /**
   * @dev Emits the `ReceivedNft` event if the transfer succeeds.
   */
  function depositNft(
    address nft_address,
    uint256 tokenId,
    bytes32 principal
  ) public {
    IERC721 nftToken = IERC721(nft_address);
    nftToken.safeTransferFrom(msg.sender, cketh_minter_main_address, tokenId);

    emit ReceivedNft(nft_address, msg.sender, tokenId, principal);
  }
}
