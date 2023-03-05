// SPDX-License-Identifier: MIT

pragma solidity ^0.8.17;

contract Escrow {
    address payable public Buyer;
    address payable public Seller;
    address public arbiter;
    uint256 public amount;
    bool public buyerApproved;
    bool public sellerApproved;
    address public owner;

    constructor() {
        owner = msg.sender;
    }

    function initiaize(
        address payable _buyer,
        address payable _seller,
        address _arbiter
    ) public payable {
        require(owner == msg.sender, "only owner can initialize");
        require(
            _buyer != _seller && _buyer != _arbiter && _seller != _arbiter,
            "Buyer, seller and arbiter can not be identical"
        );
        require(msg.value > 100 wei, "Insufficient balance");
        Buyer = _buyer;
        Seller = _seller;
        arbiter = _arbiter;
        amount = msg.value;
    }

    function approveByBuyer() public {
        require(msg.sender == Buyer, "Only buyer can approve");
        require(buyerApproved != true, "Buyer already approved");
        buyerApproved = true;
    }

    function approveBySeller() public {
        require(msg.sender == Seller, "Only buyer can approve");
        require(sellerApproved != true, "Buyer already approved");
        sellerApproved = true;
    }

    function releaseToBuyer() public {}

    function releaseToSeller() public {}

    function dispute() public {}

    function getBalance() public view returns (uint256) {}

    function reset() public {}
}

// 0x78731D3Ca6b7E34aC0F824c42a7cC18A495cabaB
// 0xAb8483F64d9C6d1EcF9b849Ae677dD3315835cb2
// 0x4B20993Bc481177ec7E8f571ceCaE8A9e22C02db
